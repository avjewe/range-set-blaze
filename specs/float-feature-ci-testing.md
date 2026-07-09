# Spec: CI/local-check coverage for the float feature flags

## Scope

This is a test/CI infrastructure change only: new `cargo test` / `cargo check` /
`wasm-pack test` invocations added to `.github/workflows/ci.yml`, `justfile`, and
`xtask/src/main.rs`. It does not touch `src/`, does not investigate or fix any bug in the
float feature itself, and is not a response to a known failure — it's establishing
reasonable, non-exhaustive test coverage for flags that currently have none or partial
coverage. If any of the new test lines fail once added, that's a separate follow-up
(a real bug found), not part of this change.

Every new test line is added to CI *and* mirrored locally (`justfile` / `cargo
check-all`) wherever that's practical, so a contributor gets the same signal before
pushing that CI would give them after. The one exception is WASM (section 4): running it
locally requires a browser/`wasmtime` setup that's already how the existing WASM checks
work today (CI-only, not part of `just check-all`), so the new float-on-WASM lines follow
that same existing precedent rather than introducing a new local/CI asymmetry.

## Background

PR #30 adds total-ordering float wrappers (`TotalF32`/`TotalF64`/`FiniteF32`/`FiniteF64`,
plus `f16`/`f128` variants on nightly) behind two feature flags:

- `total_float_experimental` — stable, `f32`/`f64` only.
- `total_float_nightly_experimental` — nightly, implies the above, adds `f16`/`f128`.

Both are experimental and off by default. Current CI (`.github/workflows/ci.yml`) and the
local equivalents (`justfile`, `cargo check-all` / `xtask/src/main.rs`) test these flags
inconsistently:

- `justfile`'s `test-stable` and `xtask`'s `check_all()` never exercise
  `total_float_experimental` at all — only `ci.yml` does. A contributor running
  `just check-all` or `cargo check-all` before pushing gets no local signal on the float
  feature.
- No-std + float is only ever `cargo check`ed (on `thumbv7m-none-eabi`), never
  `cargo test`ed — so no-std float code compiles but its behavior (`Ord` impls, NaN /
  negative-zero handling) is never actually executed.
- `total_float_experimental` is never combined with `rog_experimental` under an actual
  test run on stable — only implicitly via nightly's `--all-features`, which also pulls
  in `from_slice` and the nightly-only float flag, so a stable-only interaction bug
  between float and rog can't be isolated.
- WASM targets (`wasm32-unknown-unknown`, `wasm32-wasip1`) never build or test with either
  float flag.
- 32-bit (`i686-unknown-linux-gnu`) never tests either float flag.

## Goals

Close the coverage gaps that are cheap and likely to catch real bugs, without building a
full combinatorial matrix — this feature is experimental, so "every plausible pairing"
is enough, not "every pairing." Given that WASM is itself a 32-bit target, both 32-bit
platforms (`i686` and WASM) are treated as first-class, not deferred: float bit-pattern
and precision bugs are exactly the class of bug most likely to be width-dependent.

## Non-goals

- Testing `total_float_experimental` x `total_float_nightly_experimental` x
  `rog_experimental` x `from_slice` as an exhaustive cross product.

## Proposed changes

### 1. Fix `justfile` / `xtask` parity with `ci.yml`

Add the missing stable float test line to both local-check paths so they actually mirror
CI:

```sh
cargo test --verbose --features "std total_float_experimental"
```

- `justfile`: add to `test-stable`.
- `xtask/src/main.rs`: add to the `steps` vec in `check_all()`.

### 2. Test `total_float_experimental` + `rog_experimental` together, once

These are the two flags most likely to be turned on simultaneously by a real user. Add a
single combined stable test run (in `ci.yml`, `justfile`, and `xtask`):

```sh
cargo test --verbose --features "std rog_experimental total_float_experimental"
```

### 3. Upgrade no-std float coverage from `check` to `test`

Currently no-std float is only `cargo check`ed on the embedded target. Add a hosted
(non-embedded) no-std test run, which actually executes assertions instead of just
type-checking:

```sh
cargo test --verbose --no-default-features --features total_float_experimental
```

Add this next to the existing `cargo test --no-default-features --features
"rog_experimental"` line in the stable job, `justfile`, and `xtask`.

Leave the existing embedded `cargo check --target thumbv7m-none-eabi ... total_float_experimental`
and `... total_float_nightly_experimental` lines as-is — they still validate no-std +
no-alloc-runtime compilation on a real embedded target, which the hosted test can't cover.

### 4. Add WASM coverage for the float feature

Add float-feature test runs to the WASM job for both targets already exercised there:

```sh
# wasm32-unknown via wasm-pack (in test_wasm job)
wasm-pack test --chrome --headless -- --features total_float_experimental --verbose

# wasm32-wasip1 (in test_wasm job)
cargo test --target wasm32-wasip1 --verbose --features total_float_experimental
```

This is a deliberate addition per explicit request, since WASM has its own history of
float/NaN bit-pattern surprises (e.g. NaN canonicalization differences between engines)
that are worth catching even for an experimental feature.

### 5. Add 32-bit coverage for the float feature

`i686-unknown-linux-gnu` and WASM are both 32-bit targets, and float bit-pattern /
precision bugs are exactly the class most likely to be width-dependent (e.g. x87
extended-precision arithmetic on `i686`, differing NaN canonicalization across engines).
Rather than deferring this, add it alongside the existing 32-bit run:

```sh
cargo test --target i686-unknown-linux-gnu --verbose --features total_float_experimental
```

Add this next to the existing `cargo test --target i686-unknown-linux-gnu --verbose
--no-default-features --features "rog_experimental"` line in `test_32_bit_linux`.

### 6. Leave `total_float_nightly_experimental` covered only via `--all-features`

No isolated nightly-float-only test run is added. It's already exercised bundled with
`rog_experimental` / `from_slice` via `cargo test --all-features` on nightly, which is
sufficient for a nightly-only, doubly-experimental feature.

## Summary of new/changed lines

| Location | Change |
| --- | --- |
| `ci.yml` (`test_64_bit`, stable step) | add `std rog_experimental total_float_experimental` test; add no-std `total_float_experimental` test |
| `ci.yml` (`test_32_bit_linux` job) | add `total_float_experimental` test on `i686-unknown-linux-gnu` |
| `ci.yml` (`test_wasm` job) | add `total_float_experimental` to both wasm-pack and wasm32-wasip1 test invocations |
| `justfile` (`test-stable`) | add `std total_float_experimental`, `std rog_experimental total_float_experimental`, and no-std `total_float_experimental` lines |
| `xtask/src/main.rs` (`check_all`) | add the same steps so `cargo check-all` matches CI |
