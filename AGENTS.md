# Coding Notes for Agents

This file contains shared workspace rules for this repository. `range-set-blaze` is a
published crate (crates.io) with real downstream users, so stability, semver discipline, and
public-API quality matter more here than in an experimental project.

## Stability and API Policy

- This crate is published and versioned under semver. Treat any change to a `pub` signature,
  trait bound, or re-export as a breaking-change candidate; call it out explicitly rather than
  making it silently.
- Do not remove or rename public items. If an item is genuinely obsolete, deprecate it with
  `#[deprecated(note = "...")]` and leave removal to a deliberate major-version bump the human
  approves.
- New capabilities that are not yet API-stable belong behind a `_experimental` (or
  `_nightly_experimental`) feature flag, matching the existing `rog_experimental`,
  `total_float_experimental`, and `total_float_nightly_experimental` pattern in `Cargo.toml`.
  Do not fold experimental behavior into the default feature set.
- Keep the crate `no_std` (see `#![no_std]` in `src/lib.rs`) and `alloc`-based (not
  allocation-free) unless the user explicitly changes that goal. The `std` feature only adds
  `std`-specific trait impls/conveniences on top, it is not required for core functionality.
- Respect the documented MSRV (`rust-version` in `Cargo.toml`, currently 1.87) and `edition`.
  Don't use newer syntax/stdlib features than the MSRV allows.

## Unsafe Code

- Avoid introducing new `unsafe` blocks. The crate has a small number of existing `unsafe`
  transmutes (`src/float/total.rs`, `src/float/finite.rs`) for zero-cost layout reinterpretation
  between newtype wrappers and their primitive representation — each is narrowly scoped and
  documented at the call site. Follow that bar: if a change truly requires `unsafe`, call it out
  explicitly, keep it as narrow as possible, and explain the safety invariant in a `// SAFETY:`
  comment so the user can review it carefully.
- Do not "fix" warnings or errors by suppressing lints (`#[allow(...)]`, crate-level allow
  attributes, or similar) unless the human explicitly requests that suppression.
- If warnings are caused by obsolete code, delete or refactor the obsolete code instead of
  hiding the warning.

## Error Handling

- This is a data structure crate in the spirit of `BTreeSet`/`BTreeMap`/`HashSet`: the public API
  should not return `Result`. Invalid input (e.g. a malformed range, NaN where a total order is
  required) should fail via `assert!`/`panic!` at the call site, the same way `BTreeSet` panics
  rather than returning `Result` for programmer-error conditions. Don't introduce `Result` return
  types, including for fallible construction — prefer `Option` (single failure mode) or a panic,
  and don't implement `TryFrom`/`TryInto` just to get a `Result`-shaped constructor.
- Avoid silent clamping or best-effort fallback behavior on out-of-range or invalid input; prefer
  asserts so misuse fails fast and visibly, especially in `const fn` and hot paths where a silent
  wrong answer would be worse than a panic.
- Never use `let _ = …` to suppress a genuine `Result`. For a non-`Result` value that is
  intentionally unused, call the function as a plain statement instead of binding it to `_`.

## Testing and Local CI

- `just check-all` is the local CI gate — it mirrors what GitHub CI runs (clippy, stable tests,
  nightly tests, formatting, doc links, etc.). Run it before pushing.
- `just clippy` matches CI's exact clippy invocation (`-D clippy::all`); do not add narrower
  clippy configs that could pass locally but fail CI, and do not weaken CI's clippy flags.
- When adding a feature flag, wire it into the relevant `just`/xtask commands so CI actually
  exercises it — an experimental feature that only compiles when a human remembers to pass
  `--features` by hand is effectively untested.
- Prefer `no_run` doctests over `ignore`; use `ignore` only when truly necessary and say why.
  Always write `rust,no_run` (not bare `no_run`) in the fence. Hide setup boilerplate with a `#`
  prefix when it's required for compilation but not relevant to the reader.
- For public methods, prefer a doctest on the method itself. Where one shared example covers a
  family of methods, have each method's doc comment link to it explicitly rather than leaving
  the connection implicit.
- Do not remove debug/test code, commented-out comparison blocks, or in-progress test scaffolding
  until the underlying issue is proven fixed and the human has accepted the cleanup.

## Module Structure Convention

Do not create `mod.rs` files. This repo already follows the `src/foo.rs` + `src/foo/bar.rs`
pattern throughout (e.g. `src/float.rs` + `src/float/finite.rs`); keep new modules consistent
with it.

## Comment and TODO Conventions

- Plain `TODO` means non-blocking/future work.
- When changing code, don't remove existing `TODO` comments just because you touched nearby
  code; move them if the code moves, or append `(may no longer apply)` if you believe they're
  stale rather than deleting them outright.
- Document non-obvious invariants (e.g. why an `unsafe` transmute is sound, why a bound is
  needed for coherence) at the point of use, not just in a commit message.

## Documentation Conventions

- Use American spelling.
- When linking to module or type documentation, name the item in the link text.
- If an item comes from `crate`, `core`, or `alloc`, import it with `use` instead of a
  fully-qualified path in code; fully-qualified paths are fine in docs/comments.
- Rust getters should not use a `get_` prefix (`len()`, not `get_len()`).
- Markdown formatting: blank lines before/after lists, fenced code blocks, and headings; keep
  list marker style consistent within a file.

## Specs

Put implementation specs (`*_SPEC.md` and similar planning documents) in the `specs/` directory,
not the repo root. Every spec must include a `todo0` comment near the top reminding readers to
consider deleting the spec once the work it describes is complete, for example:

```markdown
<!-- todo0 consider deleting this spec once the work below is implemented and released. -->
```

## Release Discipline

- Do not run the real `cargo publish` (or `just publish-dry-all`'s non-dry variant). Prepare
  release notes, version bumps, and the publish command, but the actual publish step is run by
  the human.
- Always suggest a concise 1-2 line commit message when completing work, in a fenced code block.
- Treat `Cargo.lock` and dependency version bumps as deliberate, reviewable changes, not
  incidental side effects of an unrelated task.
