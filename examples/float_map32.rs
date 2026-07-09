//! Example: the `f32` counterpart to `float_maps.rs`, which does the same
//! Taylor-series term-count analysis for `f16`. There are ~4.28 billion
//! finite `f32` values -- far too many to sweep one by one the way the
//! `f16` version does (only ~61,000 of those). Instead, since the term
//! count is a monotonic, non-decreasing function of `|x|`, we binary-search
//! directly on `FiniteF32`'s own ordered representation to find each exact
//! boundary between "N terms" and "N+1 terms" -- no enumeration needed,
//! even though the ranges being searched span millions of individual
//! `f32` values. The resulting `RangeMapBlaze<FiniteF32, TermsNeeded>` ends
//! up with the same shape as the `f16` version: the same handful of
//! real-number boundaries (the term-count math doesn't care about float
//! width), just expressed as the nearest `f32` instead of the nearest
//! `f16`.
//!
//! Unlike `float_maps.rs`, this example needs no nightly feature: `f32` is
//! a stable type, so `total_float_experimental` (stable) is enough.
//!
//! See `float_maps.rs` for the ULP/epsilon/scope background this example
//! builds on without re-explaining.

use core::f64::consts::PI;
use core::num::NonZero;
use core::ops::RangeInclusive;
use range_set_blaze::{FiniteF32, Integer, RangeMapBlaze};

/// Highest number of Taylor terms we're willing to try before giving up.
const MAX_TERMS: u8 = 30;
/// Target absolute error: `f32::EPSILON` is 1 ULP near magnitude 1.
const TARGET_ERROR: f64 = f32::EPSILON as f64;

fn main() {
    let scope_upper = FiniteF32::new(PI as f32);

    // Walk term counts 1, 2, 3, ... and binary-search each one's upper
    // boundary directly, instead of sweeping every f32 in order.
    let mut entries: Vec<(RangeInclusive<FiniteF32>, TermsNeeded)> = Vec::new();
    let mut lo = FiniteF32::new(0.0);
    let mut n: u8 = 1;
    let last_hi;
    loop {
        let hi = boundary_for(n, lo, scope_upper);
        let terms = TermsNeeded::Terms(NonZero::new(n).expect("n starts at 1"));
        if n == 1 {
            // Band 1 starts at x = 0, so its mirror image touches it exactly
            // -- combine them into one symmetric range instead of two.
            entries.push((negate(hi)..=hi, terms));
        } else {
            entries.push((lo..=hi, terms));
            entries.push((negate(hi)..=negate(lo), terms));
        }
        if hi == scope_upper || n == MAX_TERMS {
            last_hi = hi;
            break;
        }
        lo = hi.next();
        n += 1;
    }
    entries.push((last_hi.next()..=FiniteF32::MAX, TermsNeeded::OutOfScope));
    entries.push((FiniteF32::MIN..=negate(last_hi).prev(), TermsNeeded::OutOfScope));

    let term_map: RangeMapBlaze<FiniteF32, TermsNeeded> = RangeMapBlaze::from_iter(entries);

    println!(
        "Taylor-series terms needed for cos(x) to guarantee an error under one f32 epsilon, for f32 in [-pi, pi]:"
    );
    for (range, terms) in term_map.range_values() {
        let (start, end) = range.into_inner();
        let label = match terms {
            TermsNeeded::OutOfScope => "out of scope".to_string(),
            TermsNeeded::Terms(n) if n.get() == 1 => "1 term".to_string(),
            TermsNeeded::Terms(n) => format!("{n} terms"),
        };
        println!(
            "  [{:>14e}, {:>14e}] -> {label}",
            start.into_inner(),
            end.into_inner(),
        );
    }
    println!(
        "\n{} disjoint ranges cover all {} finite f32 values.",
        term_map.range_values().count(),
        term_map.len()
    );
    println!(
        "Mean terms per in-scope f32 value (each value weighted equally): {:.2}",
        mean_terms(&term_map)
    );
}

/// How many Taylor terms `x` needs, or that `x` is `OutOfScope`. See
/// `float_maps.rs` for why this is an enum rather than a sentinel value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TermsNeeded {
    Terms(NonZero<u8>),
    OutOfScope,
}

/// Binary searches for the largest `FiniteF32` in `[low, high]` whose real
/// value needs at most `n` Taylor terms (see `min_terms`), using
/// `FiniteF32`'s ordered representation directly. `low`/`high` may be
/// millions of representable values apart; this still only takes about
/// 30 steps.
fn boundary_for(n: u8, low: FiniteF32, high: FiniteF32) -> FiniteF32 {
    let mut lo = low.to_ordered();
    let mut hi = high.to_ordered();
    while lo < hi {
        let mid = lo + (hi - lo + 1) / 2;
        if min_terms(f64::from(FiniteF32::from_ordered(mid).into_inner())) <= n {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    FiniteF32::from_ordered(lo)
}

/// Smallest `N` (1..=`MAX_TERMS`) for which the alternating-series
/// remainder bound, `|x|^(2N) / (2N)!`, guarantees an error under
/// `TARGET_ERROR`. Same bound as `float_maps.rs`'s `terms_needed`, just
/// without the `FiniteF16`/scope wrapping -- `boundary_for` handles that
/// part here instead.
fn min_terms(x: f64) -> u8 {
    let xx = x * x;
    let mut term_magnitude = 1.0_f64; // |x|^0 / 0!
    for n in 1..=MAX_TERMS {
        term_magnitude *= xx / f64::from(2 * u32::from(n) * (2 * u32::from(n) - 1));
        if term_magnitude <= TARGET_ERROR {
            return n;
        }
    }
    MAX_TERMS
}

/// `-x`, computed on the primitive `f32` rather than requiring `Neg` on
/// `FiniteF32` (which isn't implemented -- these wrapper types deliberately
/// expose only ordering, not arithmetic).
fn negate(x: FiniteF32) -> FiniteF32 {
    FiniteF32::new(-x.into_inner())
}

/// Plain arithmetic mean of the term count across every in-scope `f32`
/// value, each counted once. See `float_maps.rs`'s `mean_terms` for why
/// this differs from an average over the reals in `scope`'s bounds.
fn mean_terms(term_map: &RangeMapBlaze<FiniteF32, TermsNeeded>) -> f64 {
    let mut weighted_sum = 0.0;
    let mut in_scope_count = 0.0;
    for (range, terms) in term_map.range_values() {
        if let TermsNeeded::Terms(n) = terms {
            let len = FiniteF32::safe_len_to_f64_lossy(FiniteF32::safe_len(&range));
            weighted_sum += len * f64::from(n.get());
            in_scope_count += len;
        }
    }
    weighted_sum / in_scope_count
}
