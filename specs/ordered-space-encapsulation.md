<!-- todo0 consider deleting this spec once the work below is implemented and released. -->

# Encapsulate floating-point ordered-space conversions

## Goal

Keep the integer-like ordered coordinate used to implement floating-point ranges
as an implementation detail. The public API should expose floating-point values
and range operations, not the exact bit-to-ordered-space encoding.

This is especially important for `Finite`: its ordered space contains two zero
positions, while the public type canonicalizes both to `+0.0`. Consequently,
`Finite::from_ordered` is not a one-to-one inverse of `Finite::to_ordered`.

## Scope

The following user-facing methods should not be public:

- `Finite::to_ordered`
- `Finite::from_ordered`
- `Finite::try_from_ordered`

The internal conversion operations currently provided by `FiniteFloat` should
also be hidden if the implementation permits it without compromising the
public aliases or generic type usability:

- `FiniteFloat::to_ordered`
- `FiniteFloat::from_ordered`

Apply the same policy to `Total` conversions if there is no deliberate public
use case for exchanging its internal ordered coordinate. Although `Total` has a
one-to-one encoding, the encoding is still an implementation detail that could
otherwise become a compatibility obligation.

## Design requirements

1. Preserve the public floating-point range API, including constructors,
   `after`/`before`, range operations, ordering, hashing, and primitive-range
   conversion.
2. Do not promise stability for the integer type, offset, or bit layout used as
   the internal ordered coordinate.
3. Preserve `Finite`'s invariant: values are finite and zero is canonicalized to
   `+0.0`.
4. Do not add a replacement public API that exposes equivalent encoding details
   under a different name.
5. Keep the crate `no_std`, preserve the documented MSRV, and avoid new unsafe
   code.
6. Because the float APIs are experimental and under review, intentional public
   API tightening is acceptable. Document the change in release notes and call
   out the compatibility impact.

## Implementation plan

### Phase 1: hide the wrapper methods

- Remove public visibility and public documentation for the three `Finite`
  conversion methods.
- Keep their internal logic available through private helpers or direct
  `FiniteFloat` calls as needed by the implementation.
- Update tests and doctests that call these methods. Test behavior through the
  public API where possible.
- Retain coverage for finite-domain rejection and negative-zero normalization
  in module-level tests or through public constructors and range operations.

### Phase 2: hide the implementation trait's encoding

- Attempt to make `FiniteFloat` an internal implementation trait.
- If the current public generic bound prevents that, split the implementation
  responsibilities so the public type aliases do not expose the conversion
  methods. Possible approaches include a private implementation trait or
  private conversion helpers.
- Verify that public aliases such as `FiniteF32`, `FiniteF64`, `TotalF32`, and
  `TotalF64` remain usable without requiring downstream users to name internal
  traits.
- Revisit `total_float` in the same change if its public conversion methods are
  being removed.

## Compatibility notes

Removing the wrapper methods is a public API change for consumers who enabled
the experimental float feature and called them. It is intentional: retaining
them would make the current ordered encoding part of the compatibility
contract. The release notes should explain that callers should use the public
range and navigation operations instead.

If the implementation trait must remain public temporarily because of Rust's
generic-bound rules, document it as an internal sealed implementation detail
and track its removal as follow-up work. This is not complete encapsulation,
but it still removes the normal user-facing exposure.

## Verification

- `cargo fmt --check`
- `cargo test --all-features`
- `cargo clippy --all-targets --all-features -- -D clippy::all`
- `just check-all`
- Confirm with a small downstream-style compile test that the intended public
  APIs work and the ordered conversion methods are no longer accessible.
- Confirm that finite zero normalization, finite bounds, `after`/`before`, and
  range cardinality remain unchanged.
