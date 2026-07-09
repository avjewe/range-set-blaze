//! Example: the `f32` counterpart to `float_maps.rs`, which does the same
//! Taylor-series term-count analysis for `f16`. There are ~4.28 billion
//! finite `f32` values -- about 65,000 times as many as `f16`'s ~61,000 --
//! but this version sweeps every single one anyway, the same brute-force
//! way `float_maps.rs` does, just to see whether that's actually feasible.
//!
//! Unlike `float_maps.rs`, this example needs no nightly feature: `f32` is
//! a stable type, so `total_float_experimental` (stable) is enough.
//!
//! See `float_maps.rs` for the ULP/epsilon/scope background this example
//! builds on without re-explaining.

use core::f64::consts::PI;
use core::num::NonZero;
use range_set_blaze::{FiniteF32, Integer, RangeMapBlaze, RangeSetBlaze};

/// Highest number of Taylor terms we're willing to try before giving up.
const MAX_TERMS: u8 = 30;
/// Target absolute error: `f32::EPSILON` is 1 ULP near magnitude 1.
const TARGET_ERROR: f64 = f32::EPSILON as f64;

fn main() {
    // Sweep every finite f32 and collect (value, terms-needed) pairs into a
    // RangeMapBlaze -- it automatically merges neighboring f32 values that
    // need the same term count into a single range.
    let all_f32 = !RangeSetBlaze::<FiniteF32>::default(); // idiom for universe of all values
    let scope = RangeSetBlaze::from_iter([FiniteF32::new(-PI as f32)..=FiniteF32::new(PI as f32)]);
    let total = all_f32.len();

    let term_map: RangeMapBlaze<FiniteF32, TermsNeeded> = all_f32
        .iter()
        .enumerate()
        .map(|(i, x)| {
            if i % 100_000_000 == 0 {
                let pct = 100.0 * i as f64 / f64::from(total);
                eprintln!("... {} values processed ({pct:.1}%)", with_underscores(i));
            }
            (x..=x, terms_needed(x, &scope))
        })
        .collect();

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
        with_underscores(term_map.range_values().count()),
        with_underscores(term_map.len())
    );
    println!(
        "Mean terms per in-scope f32 value (each value weighted equally): {:.2}",
        mean_terms(&term_map)
    );
}

/// Formats a non-negative integer with `_` every three digits (e.g.
/// `4278190079` -> `4_278_190_079`) -- `std` has no built-in thousands
/// separator for `Display`.
fn with_underscores(n: impl core::fmt::Display) -> String {
    let digits = n.to_string();
    let mut grouped = String::with_capacity(digits.len() + digits.len() / 3);
    for (i, digit) in digits.chars().rev().enumerate() {
        if i != 0 && i % 3 == 0 {
            grouped.push('_');
        }
        grouped.push(digit);
    }
    grouped.chars().rev().collect()
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
fn terms_needed(x: FiniteF32, scope: &RangeSetBlaze<FiniteF32>) -> TermsNeeded {
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

/// Plain arithmetic mean of the term count across every in-scope `f32`
/// value, each counted once -- not an average over the reals in `scope`'s
/// bounds, which would weight every magnitude equally regardless of how
/// many (or few) `f32` values actually live there. Exact, not estimated:
/// `term_map` already holds every in-scope value's term count, coalesced
/// into a handful of ranges, so this just weights each range's term count
/// by `Integer::safe_len` (its element count) and takes the weighted
/// average -- no sampling needed.
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
