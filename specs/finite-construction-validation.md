# Spec: fix invalid-construction bugs in `Finite<T>`

Status: **implemented** (see `src/float/finite.rs`, `tests/float_tests.rs`). Scope:
`src/float/finite.rs` and `src/float/finite_float.rs` only. `Total<T>` (`src/float/total.rs`) is
untouched — it has no finiteness contract, so none of this applies to it.

## Problem

`Finite<T>`'s doc comment promises every value it holds "exclud[es] NaN, -0.0, and infinities."
`new`/`try_new` enforce that correctly. But five other public entry points built a `Finite`
directly (or transmuted into one) and bypassed both the finiteness check and the -0.0
normalization:

| Entry point | How it was built | Bypassed |
| --- | --- | --- |
| `Finite::range` | `Self(start)..=Self(end)` | tuple-struct ctor, no check |
| `Finite::ranges` | calls `range` per item | (inherited `range`'s bug) |
| `Finite::values` | `values.into_iter().map(Self)` | tuple-struct ctor, no check |
| `Finite::slice` | `transmute::<&[Primitive], &[Self]>` | no check, and can't normalize even if it wanted to (shared reference) |
| `Finite::from_ordered` | `Self(T::from_ordered(x))` | no bounds check on `x` at all |

14 tests in `tests/float_tests.rs` documented this as intentional TDD red-step failures. All 14
have been rewritten to test the fixed behavior below (some assertions were already correct and
needed no change; the `slice`/`from_ordered` ones needed real rewrites since the fix changes
what "correct" now means, including two brand-new exhaustive tests over all `f16`/`i16` bit
patterns for the constructor and `try_from_ordered` paths).

## Design principle (final decision)

Earlier drafts of this spec argued `slice_unchecked` could stay a safe fn, on the grounds that
nothing in this crate's own `unsafe` code (verified: the only `unsafe {}` blocks anywhere are
four `#[repr(transparent)]` transmutes, all layout-only, none conditioned on a `Finite`'s value)
relies on the finiteness invariant for soundness — so violating it can only produce a *logic
error* (wrong `MAX_SIZE`, a duplicated zero slot, `next()`/`prev()` landing somewhere
unexpected), never real UB, unlike `str::from_utf8_unchecked` (where the UTF-8 precondition is
load-bearing for memory safety of everything downstream that touches the `str`).

**The human overrode this and made the call explicitly**: treat `Finite<T>`'s invariant as a
public type invariant that safe code must never be able to violate, regardless of whether
today's implementation happens to only produce incorrect results rather than immediate UB. The
stated rationale, preserved here because it should guide future changes to this module too:

> This also preserves the option for this crate and downstream code to rely on the invariant in
> future safe abstractions.

In other words: the argument "nothing relies on it *yet*" is not a reason to leave the door open
via a safe fn — it's exactly the situation where a safe fn would quietly become a soundness hole
the moment someone (in this crate or downstream) *does* later write code that trusts the
invariant, without anyone having to audit or change the constructor itself. Marking the bypass
`unsafe fn` now is what keeps that future option available safely.

This settles the earlier `str::from_utf8`/`from_utf8_unchecked` vs.
`from_sorted_disjoint`/`CheckSortedDisjoint` precedent discussion: the crate now follows the
`str` shape throughout — **every unchecked constructor or zero-copy reinterpretation of a
`Finite<T>` is `unsafe fn`, gated by a `# Safety` doc comment**, and every safe entry point either
validates-and-panics or delegates to an unsafe one after establishing the precondition itself.

## Fixes (as implemented)

### 1. `new_unchecked` — new, `unsafe fn`

The single unchecked building block every other constructor in the module is now defined in
terms of:

```rust
/// # Safety
/// The caller must guarantee that:
/// - `x` is finite: not NaN, not `+/-infinity`.
/// - `x` is not `-0.0`: zero must already be canonicalized to `+0.0`.
#[must_use]
pub const unsafe fn new_unchecked(x: T::Primitive) -> Self {
    Self(x)
}
```

### 2. `try_new` / `new` — validates, then delegates

```rust
pub fn try_new(x: T::Primitive) -> Option<Self> {
    // SAFETY: `T::is_finite` rules out NaN/infinity, `T::normalize` canonicalizes -0.0.
    T::is_finite(x).then(|| unsafe { Self::new_unchecked(T::normalize(x)) })
}
```

`new()` is unchanged (`try_new(x).expect(...)`).

### 3. `from_ordered` / `try_from_ordered`

```rust
pub fn try_from_ordered(x: T::Ordered) -> Option<Self> {
    if x < T::MIN_ORDERED || x > T::MAX_ORDERED {
        return None;
    }
    // SAFETY: the bounds check rules out NaN/+-infinity; T::normalize canonicalizes -0.0.
    Some(unsafe { Self::new_unchecked(T::normalize(T::from_ordered(x))) })
}

pub fn from_ordered(x: T::Ordered) -> Self {
    Self::try_from_ordered(x)
        .expect("Finite type requires an ordered value within the finite range")
}
```

`MIN_ORDERED`/`MAX_ORDERED` already existed on `FiniteFloat` and already bound exactly the
finite range (confirmed: `MIN`/`MAX` are the largest-magnitude finite values, so their ordered
positions bracket every finite value and exclude both infinities in `total_cmp` order — also
independently confirmed by the `tf64_categories` test's comment: `NaN < -Inf < -0.0 < 0.0 < Inf
< NaN`). The internal, still-infallible `FiniteFloat::from_ordered` trait method is unchanged —
its only other caller (`inclusive_end_from_start` / `start_from_inclusive_end`) only ever
invokes it with values already proven in-domain via its own `debug_assert`-checked precondition.
Only the public, user-facing `Finite::from_ordered` needed the new check.

### 4. `range` / `ranges`

```rust
pub fn range(range: RangeInclusive<T::Primitive>) -> RangeInclusive<Self> {
    let (start, end) = range.into_inner();
    Self::new(start)..=Self::new(end)
}
```

`ranges()` needed no change — it already just calls `range()` per item.

### 5. `values`

```rust
pub fn values<I>(values: I) -> impl Iterator<Item = Self>
where
    I: IntoIterator<Item = T::Primitive>,
{
    values.into_iter().map(Self::new)
}
```

### 6. `slice` (safe, validates) / `slice_unchecked` (new, `unsafe fn`)

```rust
/// # Panics
/// Panics if any element is not finite, or is `-0.0`.
#[must_use]
pub fn slice(values: &[T::Primitive]) -> &[Self] {
    assert!(
        values.iter().all(|&v| T::is_finite(v) && !T::is_neg_zero(v)),
        "Finite type requires finite, non-negative-zero values"
    );
    // SAFETY: just validated every element is finite and not -0.0.
    unsafe { Self::slice_unchecked(values) }
}

/// # Safety
/// The caller must guarantee every element of `values` is finite and not `-0.0`. Because the
/// returned slice is a live view over the same memory (not a copy), there is no opportunity to
/// normalize `-0.0` even if the caller wanted to.
#[must_use]
pub const unsafe fn slice_unchecked(values: &[T::Primitive]) -> &[Self] {
    // SAFETY: Finite is #[repr(transparent)] over T::Primitive; the caller is responsible for
    // the value-level invariant per the safety doc above.
    unsafe { core::mem::transmute::<&[T::Primitive], &[Self]>(values) }
}
```

`slice()` keeps the name most callers reach for and is now safe-by-default (matches `new()`'s
panic-on-bad-input contract); it's O(n) to validate but still allocation-free. `slice_unchecked`
preserves the original zero-cost, `const fn` behavior for callers who have independently
established the safety precondition.

`primitive_slice` (the reverse direction, `Finite` → primitive) needed no change: a `Finite`
slice's backing data is already guaranteed valid by construction, so viewing it as primitives
loses no invariant, and it can't be used to construct a new invalid `Finite`.

## API-surface note (per AGENTS.md's semver policy)

- `from_ordered` changed from infallible to panicking on invalid input. Same signature, new
  panic path — a bug fix, not a signature break, but worth calling out since it's a behavior
  change on a stable-looking name.
- `slice()` changed from `const fn` (zero-cost, no validation) to a regular `fn` that validates
  (O(n), no alloc) and can now panic. `new_unchecked` and `slice_unchecked` are new additions
  (both `unsafe fn`) that preserve the old zero-cost, unchecked behavior for callers who need it
  and can uphold the safety precondition themselves.
- `try_from_ordered` is a new, safe addition.
- All of this lives behind `total_float_experimental` / `total_float_nightly_experimental`,
  which AGENTS.md already carves out as the "not yet API-stable" tier.

## Tests (`tests/float_tests.rs`) — implemented

- `finite_range_rejects_nan_start[_f32/_f128]`, `finite_range_rejects_infinite_end`,
  `finite_values_rejects_nan`, `finite_values_normalizes_negative_zero[_f32/_f128]` — unchanged,
  already asserted the fixed behavior.
- `finite_slice_rejects_nan[_f32/_f128]`, `finite_slice_rejects_negative_zero` —
  `#[should_panic(expected = "Finite type requires finite, non-negative-zero values")]` against
  `slice()`.
- `finite_slice_unchecked_bypasses_validation[_f32/_f128]` — each wraps
  `unsafe { Finite*::slice_unchecked(&[NAN]) }` in a `// SAFETY:` comment explaining the
  deliberate violation, documenting that the escape hatch still exists under its new,
  `unsafe`-gated name.
- `finite_f16_exhaustive_bit_patterns_via_constructors` (nightly-only, replaces
  `finite_f16_exhaustive_bit_patterns_via_bypass_constructors`) — walks all 65536 `f16` bit
  patterns; finite non-(-0.0) values must round-trip identically through `values`/`range`/
  `slice`; `-0.0` must normalize through `values`/`range` and panic through `slice` (can't
  normalize a view); everything else (NaN, +/-infinity) must panic through all three. Uses a
  temporary silent panic hook plus `std::panic::catch_unwind` for the panic-path assertions.
- `finite_f16_exhaustive_try_from_ordered` (nightly-only, replaces
  `finite_f16_exhaustive_from_ordered_rejects_out_of_domain`) — walks all 65536 `i16` ordered
  values; `try_from_ordered` must return `Some` (finite, non-negative-zero) iff the ordered
  value is within `FiniteF16::MIN.to_ordered()..=FiniteF16::MAX.to_ordered()`, `None` otherwise.
- `finite_f32_from_ordered_rejects_nan_ordered` (replaces
  `finite_f32_from_ordered_accepts_nan_ordered_bug`) —
  `#[should_panic(expected = "Finite type requires an ordered value within the finite range")]`
  against `from_ordered`, plus a companion `finite_f32_try_from_ordered_rejects_nan_ordered`
  asserting `try_from_ordered` returns `None` for the same input.

Verified: `cargo test --features total_float_experimental` (stable) and
`cargo +nightly test --features total_float_nightly_experimental` (nightly) both pass in full,
including doctests and `just clippy`.
