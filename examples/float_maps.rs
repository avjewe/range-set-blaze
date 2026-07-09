//! Example: approximate `cos` for `f16` with a plain Taylor series, and use
//! an exhaustive sweep over every finite `f16` value -- there are only
//! about 61,000 of them -- to discover how many series terms each one needs
//! to guarantee an error under one `f32` ULP (`f32::EPSILON`), for the
//! values that matter: `x` in `scope` (see `main`).
//!
//! "ULP" stands for "unit in the last place": the gap between one
//! representable float and the very next one. It's not a fixed number --
//! it grows as the magnitude of the value grows, since floats pack more
//! precision near zero than far from it. This crate's `.next()`/`.prev()`
//! step by exactly 1 ULP. Here we use a fixed target error of `f32::EPSILON`
//! (1 ULP near magnitude 1) rather than "1 ULP of whatever `cos(x)` happens
//! to be": near `cos`'s zero-crossings the true value is tiny, so its own
//! ULP is tiny too, and demanding that precision would be unreachable.
//!
//! The term count per value is found from the textbook alternating-series
//! remainder bound, `|x|^(2N) / (2N)!`, rather than by empirically comparing
//! against a reference `cos` -- that bound is a pure function of `|x|`, so
//! it can't flicker between "N terms" and "N+1 terms" the way a
//! rounding-sensitive empirical search would right at the boundary. The
//! result is captured as a `RangeMapBlaze<FiniteF16, TermsNeeded>`, which
//! coalesces neighboring `f16` values that need the same term count into a
//! single, clean, non-overlapping range.
//!
//! Why only a limited `scope`? A raw Taylor series only converges quickly
//! near zero. Real math libraries handle an `x` of any size by first
//! folding it into a bounded range using `cos`'s periodicity and evenness
//! (`cos(-x) = cos(x)`, so strictly only `[0, pi]` is ever necessary).
//! Implementing that fold is a separate concern from the point of this
//! demo, so it's left out: we simply restrict our attention to `f16` values
//! already inside `scope` and report everything else as a single
//! `out of scope` bucket. `scope` is a `RangeSetBlaze<FiniteF16>` built once
//! in `main` and threaded into `terms_needed`, which decides both "is this
//! in scope" and "how many terms" together -- so widening or narrowing the
//! domain (as we've done a few times already) only means editing where
//! `scope` is built.
//! The point here is the `RangeMapBlaze`, not the numerics.
//!
//! Note: this example only builds with `--features
//! total_float_nightly_experimental` (see `required-features` in
//! `Cargo.toml`), so the whole file can assume that feature is on -- no
//! `#[cfg(...)]` needed anywhere below.

#![feature(f16)]

use core::f64::consts::PI;
use core::num::NonZero;
use range_set_blaze::{FiniteF16, Integer, RangeMapBlaze, RangeSetBlaze};

/// Highest number of Taylor terms we're willing to try before giving up.
const MAX_TERMS: u8 = 30;
/// Target absolute error: `f32::EPSILON` is 1 ULP near magnitude 1 -- see
/// the module doc for why a fixed target beats "1 ULP of the reference".
const TARGET_ERROR: f64 = f32::EPSILON as f64;

fn main() {
    // Sweep every finite f16 and collect (value, terms-needed) pairs into a
    // RangeMapBlaze -- it automatically merges neighboring f16 values that
    // need the same term count into a single range.
    let all_f16 = !RangeSetBlaze::<FiniteF16>::default(); // idiom for universe of all values
    let scope = RangeSetBlaze::from_iter([FiniteF16::new(-PI as f16)..=FiniteF16::new(PI as f16)]);

    let term_map: RangeMapBlaze<FiniteF16, TermsNeeded> = all_f16
        .iter()
        .map(|x| (x..=x, terms_needed(x, &scope)))
        .collect();

    println!(
        "Taylor-series terms needed for cos(x) to guarantee 1 ULP accuracy, for f16 in [-pi, pi]:"
    );
    for (range, terms) in term_map.range_values() {
        let (start, end) = range.into_inner();
        let label = match terms {
            TermsNeeded::OutOfScope => "out of scope".to_string(),
            TermsNeeded::Terms(n) => format!("{n} terms"),
        };
        println!(
            "  [{:>12}, {:>12}] -> {label}",
            start.into_inner() as f32,
            end.into_inner() as f32,
        );
    }
    println!(
        "\n{} disjoint ranges cover all {} finite f16 values.",
        term_map.range_values().count(),
        term_map.len()
    );
    println!(
        "Expected terms for x ~ Uniform over the in-scope f16 values: {:.2}",
        expected_terms(&term_map)
    );
}

/// How many Taylor terms `x` needs, or that `x` is `OutOfScope` (not a
/// member of `scope`). An enum (rather than a sentinel value like
/// `u32::MAX`) makes the "no term count applies" case something callers
/// have to handle explicitly instead of something they can accidentally do
/// arithmetic on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TermsNeeded {
    /// Term counts are always at least 1 and never exceed `MAX_TERMS` (30),
    /// so `NonZero<u8>` fits with room to spare -- no need for `u32`.
    Terms(NonZero<u8>),
    OutOfScope,
}

/// Decides both whether `x` is a member of `scope` and, if so, how many
/// terms it needs: smallest `N` (1..=`MAX_TERMS`) for which the
/// alternating-series remainder bound, `|x|^(2N) / (2N)!`, guarantees an
/// error under `TARGET_ERROR`.
fn terms_needed(x: FiniteF16, scope: &RangeSetBlaze<FiniteF16>) -> TermsNeeded {
    if !scope.contains(x) {
        return TermsNeeded::OutOfScope;
    }
    let x = f64::from(x.into_inner());
    let xx = x * x;
    let mut term_magnitude = 1.0_f64; // |x|^0 / 0!
    for n in 1..=MAX_TERMS {
        let n = NonZero::new(n).expect("loop starts at 1, so n is never zero");
        term_magnitude *= xx / f64::from(2 * u32::from(n.get()) * (2 * u32::from(n.get()) - 1));
        if term_magnitude <= TARGET_ERROR || n.get() == MAX_TERMS {
            return TermsNeeded::Terms(n);
        }
    }
    unreachable!("loop always returns once n reaches MAX_TERMS")
}

/// Expected Taylor term count for `x` drawn uniformly from the finite `f16`
/// values that are in scope. Exact, not estimated: `term_map` already holds
/// every in-scope value's term count, coalesced into a handful of ranges,
/// so this just weights each range's term count by `Integer::safe_len` (its
/// element count) and takes the weighted average -- no sampling needed.
fn expected_terms(term_map: &RangeMapBlaze<FiniteF16, TermsNeeded>) -> f64 {
    let mut weighted_sum = 0.0;
    let mut in_scope_count = 0.0;
    for (range, terms) in term_map.range_values() {
        if let TermsNeeded::Terms(n) = terms {
            let len = FiniteF16::safe_len_to_f64_lossy(FiniteF16::safe_len(&range));
            weighted_sum += len * f64::from(n.get());
            in_scope_count += len;
        }
    }
    weighted_sum / in_scope_count
}
