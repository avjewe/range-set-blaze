//! Example: approximate `cos` for `f16` with a plain Taylor series, and use
//! an exhaustive sweep over every finite `f16` value -- there are only
//! about 61,000 of them -- to discover how many series terms are needed to
//! land within 1 ULP of a reference value.
//!
//! "ULP" stands for "unit in the last place": the gap between one
//! representable float and the very next one. It's not a fixed number --
//! it grows as the magnitude of the value grows, since floats pack more
//! precision near zero than far from it. Saying an approximation is "within
//! 1 ULP" means it's as close as floating-point math can represent, i.e.
//! there's no representable value strictly between the approximation and
//! the true answer. This crate's `.next()`/`.prev()` step by exactly 1 ULP.
//!
//! The answer -- how many Taylor terms each input needs -- is captured as a
//! `RangeMapBlaze<FiniteF16, u32>`, which coalesces neighboring `f16`
//! values that need the same term count into a single clean, non-overlapping
//! range.
//!
//! Real math libraries use range reduction plus a minimax polynomial rather
//! than a raw Taylor series -- that's a separate optimization problem over
//! the *polynomial*, not the *input space*, so it's left out on purpose.
//! The point here is the `RangeMapBlaze`, not the numerics.

#![cfg_attr(feature = "total_float_nightly_experimental", feature(f16))]

#[cfg(feature = "total_float_nightly_experimental")]
use range_set_blaze::{FiniteF16, RangeMapBlaze, RangeSetBlaze, TotalF32};

/// Highest number of Taylor terms we're willing to try before giving up.
#[cfg(feature = "total_float_nightly_experimental")]
const MAX_TERMS: u32 = 30;
/// How close (in `f32` ULPs) the Taylor approximation must land to count as "good enough".
#[cfg(feature = "total_float_nightly_experimental")]
const ULP_TOLERANCE: i64 = 1;

fn main() {
    #[cfg(feature = "total_float_nightly_experimental")]
    run();

    #[cfg(not(feature = "total_float_nightly_experimental"))]
    println!(
        "This example needs f16 support: run with \
         `cargo run --example float_maps --features total_float_nightly_experimental`"
    );
}

#[cfg(feature = "total_float_nightly_experimental")]
fn run() {
    let term_map = build_taylor_term_map();

    println!("Taylor-series terms needed for cos(x) to land within 1 ULP, by f16 range:");
    for (range, terms) in term_map.range_values() {
        let (start, end) = range.into_inner();
        let flag = if *terms == MAX_TERMS { "  (gave up)" } else { "" };
        println!(
            "  [{:>12}, {:>12}] -> {terms:>2} terms{flag}",
            start.into_inner() as f32,
            end.into_inner() as f32,
        );
    }
    println!(
        "\n{} disjoint ranges cover all {} finite f16 values.",
        term_map.range_values().count(),
        term_map.len()
    );
}

/// For every finite `f16`, find the fewest Taylor terms whose result --
/// computed and compared in `f32` -- lands within `ULP_TOLERANCE` of
/// `f64::cos`, falling back to `MAX_TERMS` as a "gave up" sentinel.
/// Collecting the `(value, terms)` pairs into a `RangeMapBlaze`
/// automatically merges runs of neighboring `f16` values that need the
/// same term count into a single range.
#[cfg(feature = "total_float_nightly_experimental")]
fn build_taylor_term_map() -> RangeMapBlaze<FiniteF16, u32> {
    let all_f16 = RangeSetBlaze::from_iter([FiniteF16::MIN..=FiniteF16::MAX]);
    all_f16
        .iter()
        .map(|x| {
            let x32 = x.into_inner() as f32;
            let reference = f64::cos(f64::from(x32)) as f32;
            let terms = (1..=MAX_TERMS)
                .find(|&terms| ulp_distance(taylor_cos(x32, terms), reference) <= ULP_TOLERANCE)
                .unwrap_or(MAX_TERMS);
            (x..=x, terms)
        })
        .collect()
}

/// Distance between two `f32` values, in ULPs, via the crate's own total order.
/// Uses `TotalF32` (not `FiniteF32`) because a runaway Taylor series can
/// produce NaN or infinity, and those still need a well-defined distance.
#[cfg(feature = "total_float_nightly_experimental")]
fn ulp_distance(a: f32, b: f32) -> i64 {
    let a = i64::from(TotalF32::new(a).to_ordered());
    let b = i64::from(TotalF32::new(b).to_ordered());
    (a - b).abs()
}

/// `cos(x) = sum_k (-1)^k x^(2k) / (2k)!`, evaluated in `f32` using the first `terms` terms.
#[cfg(feature = "total_float_nightly_experimental")]
fn taylor_cos(x: f32, terms: u32) -> f32 {
    let x2 = x * x;
    let mut term = 1.0_f32;
    let mut sum = 1.0_f32;
    for k in 1..terms {
        term *= -x2 / (2 * k * (2 * k - 1)) as f32;
        sum += term;
    }
    sum
}
