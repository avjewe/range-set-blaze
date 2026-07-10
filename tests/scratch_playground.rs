//! TODO000: delete this scratch file and the matching .vscode/settings.json entry when done
use range_set_blaze::RangeSetBlaze;
use range_set_blaze::finite::ff32;
use range_set_blaze::total::tf64;
use range_set_blaze::{FiniteF32, FiniteF64, RangeMapBlaze, TotalF32, TotalF64};

/// Playground for the `from_ordered` bug: `FiniteF32` promises to only ever
/// hold finite, non-NaN, non-negative-zero values -- `new`/`try_new` enforce
/// that. But `from_ordered` is public, infallible, and does zero checking.
/// `Ordered` is just `i32` for f32, and `TotalF32` (which has no finiteness
/// restriction) uses the exact same `Ordered` encoding -- so we can borrow a
/// `TotalF32` to manufacture "the Ordered position of NaN/-0.0/+Infinity"
/// and hand it straight to `FiniteF32::from_ordered` to see what comes out.
#[test]
fn scratch7_from_ordered_bug() {
    let nan_ordered = TotalF32::new(f32::NAN).to_ordered();
    let broken = FiniteF32::from_ordered(nan_ordered);
    println!("FiniteF32::from_ordered({nan_ordered}) = {broken:?} -- supposed to be impossible!");
    println!(
        "  .into_inner().is_nan() = {}",
        broken.into_inner().is_nan()
    );

    let neg_inf_ordered = TotalF32::new(f32::NEG_INFINITY).to_ordered();
    let broken_inf = FiniteF32::from_ordered(neg_inf_ordered);
    println!(
        "FiniteF32::from_ordered({neg_inf_ordered}) = {broken_inf:?} -- also supposed to be impossible!"
    );

    let neg_zero_ordered = TotalF32::new(-0.0).to_ordered();
    let broken_neg_zero = FiniteF32::from_ordered(neg_zero_ordered);
    println!(
        "FiniteF32::from_ordered({neg_zero_ordered}) = {broken_neg_zero:?}, is_sign_negative = {}",
        broken_neg_zero.into_inner().is_sign_negative()
    );

    // Compare: the *safe* constructor for the same underlying value refuses
    // NaN outright and normalizes -0.0 away, exactly as documented.
    println!(
        "for reference, FiniteF32::try_new(f32::NAN) = {:?}",
        FiniteF32::try_new(f32::NAN)
    );
    println!(
        "for reference, FiniteF32::new(-0.0) = {:?} (normalized to +0.0)",
        FiniteF32::new(-0.0)
    );
}

/// Scratch-only naming experiment: `.after()`/`.before()` as an alternative
/// spelling for `.next()`/`.prev()`. Lives only in this playground file, not
/// in the crate — just delegates to the real methods so the idiom
/// `x.after()..=y.before()` can be tried out.
trait BeforeAfter {
    #[must_use]
    fn after(self) -> Self;
    #[must_use]
    fn before(self) -> Self;
}

macro_rules! impl_before_after {
    ($($ty:ty),* $(,)?) => {
        $(
            impl BeforeAfter for $ty {
                fn after(self) -> Self {
                    self.next()
                }
                fn before(self) -> Self {
                    self.prev()
                }
            }
        )*
    };
}

impl_before_after!(FiniteF32, FiniteF64, TotalF32, TotalF64);

#[test]
fn scratch() {
    let set = RangeSetBlaze::from_iter([ff32(2.0)..=ff32(2.0)]);
    println!("set: {set:?}");

    let complement = !set;
    println!("complement: {complement:?}");
    println!("count: {}", complement.len());
}

#[test]
fn scratch2() {
    let epsilon = ff32(f32::EPSILON);
    println!("epsilon: {epsilon:?}");

    let after = epsilon.after();
    println!("after: {after:?}");
}

#[test]
fn scratch3() {
    let zero = ff32(-0.0);
    println!("zero: {zero:?}");

    let after = zero.after();
    println!("after: {after:?}");

    println!("is epsilon? {}", after == ff32(f32::EPSILON));
}

#[test]
fn scratch4() {
    let one = ff32(1.0);
    println!("one: {one:?}");

    let after = one.after();
    println!("after: {after:?}");

    println!(
        "after == 1.0 + epsilon? {}",
        after == ff32(1.0 + f32::EPSILON)
    );
}

#[test]
fn scratch5_before_after() {
    // The idiom under test: exclusive bounds (2.0, 5.0) expressed as an
    // inclusive range via .after()/.before() instead of .next()/.prev().
    let set = RangeSetBlaze::from_iter([ff32(2.0).after()..=ff32(5.0).before()]);
    println!("open interval (2.0, 5.0) as Finite: {set:?}");
    assert!(!set.contains(ff32(2.0)));
    assert!(set.contains(ff32(2.0).after()));
    assert!(set.contains(ff32(5.0).before()));
    assert!(!set.contains(ff32(5.0)));

    // Same idiom on TotalF32, where "after"/"before" should still read
    // sensibly even though the domain includes NaN, -0.0, and infinities.
    let nan = TotalF32::new(f32::NAN);
    println!("nan: {nan:?}, after(nan): {:?}", nan.after());
    assert_eq!(nan.after().before(), nan);
}

#[test]
fn scratch6_tiny() {
    // TINY, per std's own next_up()/next_down() docs: "the smallest
    // representable positive f32" — i.e. the smallest positive *subnormal*,
    // not f32::MIN_POSITIVE (which is the smallest *normal* value).
    let tiny = f32::from_bits(1);
    println!("TINY = {tiny:e} (bits: {:#010x})", tiny.to_bits());
    println!("TINY == 0f32.next_up()? {}", tiny == 0f32.next_up());
    println!(
        "TINY < f32::MIN_POSITIVE (smallest normal, {:e})? {}",
        f32::MIN_POSITIVE,
        tiny < f32::MIN_POSITIVE
    );
    println!("f32::EPSILON / TINY = {:e}", f32::EPSILON / tiny);

    // In this crate's total order, stepping .after() from zero lands on
    // TINY too — and it does so whether you start from +0.0 or -0.0, since
    // Finite normalizes -0.0 away (see scratch3).
    assert_eq!(ff32(0.0).after(), ff32(tiny));
    assert_eq!(ff32(-0.0).after(), ff32(tiny));

    // TINY is finite, so .before() undoes it back to zero.
    assert_eq!(ff32(tiny).before(), ff32(0.0));

    // -TINY is the mirror image: the greatest value below zero.
    let neg_tiny = -tiny;
    println!("-TINY = {neg_tiny:e}");
    assert_eq!(ff32(0.0).before(), ff32(neg_tiny));
    assert_eq!(ff32(neg_tiny).after(), ff32(0.0));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Category {
    NaN,
    NegInfinity,
    PosInfinity,
    MinusZero,
    Normal,
}

#[test]
fn scratch8_tf64_categories() {
    // Build up a category map from system constants alone: NaN, Infinity,
    // "minus zero", and Normal, using TotalF64's total_cmp-based order:
    //   MIN (most-negative NaN) < ... < NEG_INFINITY < ... < -0.0 < 0.0 < ... < INFINITY < ... < MAX (most-positive NaN)
    // This is not exhaustive (it treats every other finite value as "Normal"
    // regardless of subnormal/normal-in-the-IEEE-sense distinctions), but it
    // shows how far you can get from just NEG_INFINITY/INFINITY/MIN/MAX/0.0/-0.0.
    // Overlapping ranges have right-to-left precedence (later entries win),
    // so we can build this up coarse-to-fine instead of carving out exact
    // gaps by hand: start with everything NaN, carve out the finite range
    // between the infinities as Normal, then punch in the infinities and
    // -0.0 as exceptions on top.
    let category_map = RangeMapBlaze::from_iter([
        (TotalF64::MIN..=TotalF64::MAX, Category::NaN), // everything else is NaN by default
        (tf64(f64::NEG_INFINITY)..=tf64(f64::INFINITY), Category::Normal), // everything between the infinities is Normal, inclusive (?!)
        (tf64(f64::NEG_INFINITY)..=tf64(f64::NEG_INFINITY), Category::NegInfinity), // carve out the infinities as exceptions
        (tf64(f64::INFINITY)..=tf64(f64::INFINITY), Category::PosInfinity), // carve out the infinities as exceptions
        (tf64(-0.0)..=tf64(-0.0), Category::MinusZero), // carve out -0.0 as an exception
    ]);

    for (range, category) in category_map.range_values() {
        let (start, end) = (range.start().into_inner(), range.end().into_inner());
        println!(
            "{start:e} (0x{:016x}) ..= {end:e} (0x{:016x}) -> {category:?}",
            start.to_bits(),
            end.to_bits(),
        );
    }

    // Spot-check a handful of values land where expected.
    assert_eq!(category_map.get(tf64(f64::NAN)), Some(&Category::NaN));
    assert_eq!(category_map.get(tf64(-f64::NAN)), Some(&Category::NaN));
    assert_eq!(
        category_map.get(tf64(f64::NEG_INFINITY)),
        Some(&Category::NegInfinity)
    );
    assert_eq!(
        category_map.get(tf64(f64::INFINITY)),
        Some(&Category::PosInfinity)
    );
    assert_eq!(category_map.get(tf64(-0.0)), Some(&Category::MinusZero));
    assert_eq!(category_map.get(tf64(0.0)), Some(&Category::Normal));
    assert_eq!(category_map.get(tf64(1.0)), Some(&Category::Normal));
    assert_eq!(
        category_map.get(tf64(f64::MIN_POSITIVE)),
        Some(&Category::Normal)
    );
    assert_eq!(category_map.get(tf64(-1.5e300)), Some(&Category::Normal));
}
