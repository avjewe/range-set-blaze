//! One can approximate cosine with the Taylor series
//! `cos(x) ≈ 1 - x^2/2! + x^4/4! - x^6/6! + x^8/8! - ...`.
//!
//! But how many terms are actually needed? This example sweeps every finite
//! `f32` value in `[-pi, pi]` (~2.16 billion values), computes the smallest
//! number of Taylor terms whose remainder bound guarantees an error below
//! `TARGET_ERROR` (1e-7), and tabulates the results in a
//! `RangeMapBlaze<FiniteF32, u8>`.
//!
//! Surprisingly, most representable `f32` values need only one term (`1`) or
//! two terms (`1 - x*x/2`). The resulting `RangeMapBlaze` acts as a compact
//! dispatch table: a lookup tells you how many Taylor terms to evaluate for
//! each input range. Averaged uniformly over representable `f32` values in
//! `[-pi, pi]`, only about 1.2 terms are needed.
//!
//! Real math libraries go even further. They first reduce the input to a
//! smaller interval (using cosine's periodicity and symmetry) and then use
//! carefully optimized minimax polynomials rather than a Taylor series.
//! This example intentionally uses the simpler Taylor series so the focus
//! remains on how `RangeMapBlaze` can discover and represent an optimal
//! dispatch table.

use core::f32::consts::PI;
use range_set_blaze::{FiniteF32, Integer, RangeMapBlaze, RangeSetBlaze, finite::ff32};
use rayon::prelude::*;
use std::thread::available_parallelism;
use thousands::Separable;

fn main() {
    /// Target absolute error; can leave `taylor`/`std` disagreeing in the last printed digit (display-rounding, not a bug).
    const TARGET_ERROR: f64 = 1e-7;

    /// Smallest `N` (1..=`u8::MAX`) for which the remainder bound
    /// `|x|^(2N) / (2N)!` guarantees error under `TARGET_ERROR`. Assumes `x`
    /// is already in scope.
    fn terms_needed(x: FiniteF32) -> u8 {
        let x = f64::from(x.into_inner());
        let xx = x * x;
        let mut term_magnitude = 1.0_f64; // |x|^0 / 0!
        for n in 1..=u8::MAX {
            term_magnitude *= xx / f64::from(2 * u32::from(n) * (2 * u32::from(n) - 1));
            if term_magnitude <= TARGET_ERROR || n == u8::MAX {
                return n;
            }
        }
        unreachable!("loop always returns once n reaches u8::MAX")
    }

    let scope = RangeSetBlaze::from_iter([ff32(-PI)..=ff32(PI)]);

    // Split scope across all available cores; each thread sweeps its own
    // chunk into a local RangeMapBlaze, then `|` (union) merges them --
    // cheap here since the chunks are domain-disjoint by construction.
    let num_chunks = available_parallelism().map_or(1, |n| n.get());
    let term_map: RangeMapBlaze<FiniteF32, u8> = chunks(&scope, num_chunks)
        .into_par_iter()
        .map(|chunk| chunk.iter().map(|x| (x, terms_needed(x))).collect())
        .reduce(RangeMapBlaze::new, |a, b| a | b);

    println!(
        "Taylor-series terms needed for cos(x) to guarantee an error under one f32 epsilon, for f32 in [-pi, pi]:"
    );
    for (range, n) in term_map.range_values() {
        let (start, end) = (range.start().into_inner(), range.end().into_inner());
        let mid = (start + end) / 2.0;
        let taylor = taylor_cos(mid, *n);
        let std = mid.cos();
        println!(
            "  [{start:>14e}, {end:>14e}] -> {n} term(s)  (cos(mid): taylor={taylor:.7}, std={std:.7})",
        );
    }
    println!(
        "\n{} disjoint ranges cover all {} in-scope f32 values.",
        term_map.range_values().count().separate_with_underscores(),
        term_map.len().separate_with_underscores()
    );
    println!(
        "Mean terms per in-scope f32 value (each value weighted equally): {:.2}",
        mean_terms(&term_map)
    );
}

/// Splits `scope` (a single contiguous range) into `n` (or fewer, if `scope`
/// is smaller) contiguous, non-overlapping `RangeSetBlaze`s of nearly-equal
/// `safe_len`, any remainder spread one-per-chunk over the first chunks.
fn chunks(scope: &RangeSetBlaze<FiniteF32>, n: usize) -> Vec<RangeSetBlaze<FiniteF32>> {
    let n = (n as u32).clamp(1, scope.len());
    let (base_len, remainder) = (scope.len() / n, scope.len() % n);

    let mut start = scope.first().expect("scope is non-empty");
    (0..n)
        .map(|i| {
            let len = base_len + u32::from(i < remainder);
            let end = start.inclusive_end_from_start(len);
            let chunk = RangeSetBlaze::from_iter([start..=end]);
            start = end.checked_next().unwrap_or(end); // unused after the last chunk
            chunk
        })
        .collect()
}

/// `cos(x)` via `terms` terms of its Taylor series, for display purposes
/// only -- `terms_needed` never evaluates the series itself.
fn taylor_cos(x: f32, terms: u8) -> f32 {
    let xx = x * x;
    let mut term = 1.0_f32;
    let mut sum = 1.0_f32;
    for k in 1..terms {
        term *= -xx / (2 * u32::from(k) * (2 * u32::from(k) - 1)) as f32;
        sum += term;
    }
    sum
}

/// Mean term count across in-scope `f32` values, each counted once (not an
/// average over the reals, which would over-weight sparse magnitudes).
/// Exact: weights each range's term count by its `Integer::safe_len`.
fn mean_terms(term_map: &RangeMapBlaze<FiniteF32, u8>) -> f64 {
    let mut weighted_sum = 0.0;
    let mut total_count = 0.0;
    for (range, n) in term_map.range_values() {
        let len = FiniteF32::safe_len_to_f64_lossy(FiniteF32::safe_len(&range));
        weighted_sum += len * f64::from(*n);
        total_count += len;
    }
    weighted_sum / total_count
}
