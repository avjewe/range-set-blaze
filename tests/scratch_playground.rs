//! TODO000: delete this scratch file and the matching .vscode/settings.json entry when done
#![cfg(feature = "total_float_experimental")]
use range_set_blaze::RangeSetBlaze;
use range_set_blaze::finite::{FiniteRangeExt, FiniteSliceExt, ff32};
use range_set_blaze::total::TotalSliceExt;
use range_set_blaze::{FiniteF32, FiniteF64, TotalF32};

#[test]
#[ignore = "scratch playground, not a real test — run explicitly with `cargo test --test scratch_playground -- --ignored`"]
fn scratch7() {
    println!(
        "FiniteF32::try_new(f32::NAN) = {:?}",
        FiniteF32::try_new(f32::NAN)
    );
    println!("FiniteF32::new(-0.0) = {:?}", FiniteF32::new(-0.0));
}

#[test]
#[ignore = "scratch playground, not a real test — run explicitly with `cargo test --test scratch_playground -- --ignored`"]
fn scratch() {
    let set = RangeSetBlaze::from_iter([ff32(2.0)..=ff32(2.0)]);
    println!("set: {set:?}");

    let complement = !set;
    println!("complement: {complement:?}");
    println!("count: {}", complement.len());
}

#[test]
#[ignore = "scratch playground, not a real test — run explicitly with `cargo test --test scratch_playground -- --ignored`"]
fn scratch2() {
    let epsilon = ff32(f32::EPSILON);
    println!("epsilon: {epsilon:?}");

    let after = epsilon.after();
    println!("after: {after:?}");
}

#[test]
#[ignore = "scratch playground, not a real test — run explicitly with `cargo test --test scratch_playground -- --ignored`"]
fn scratch3() {
    let zero = ff32(-0.0);
    println!("zero: {zero:?}");

    let after = zero.after();
    println!("after: {after:?}");

    println!("is epsilon? {}", after == ff32(f32::EPSILON));
}

#[test]
#[ignore = "scratch playground, not a real test — run explicitly with `cargo test --test scratch_playground -- --ignored`"]
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
#[ignore = "scratch playground, not a real test — run explicitly with `cargo test --test scratch_playground -- --ignored`"]
fn scratch5_before_after() {
    // The idiom under test: exclusive bounds (2.0, 5.0) expressed as an
    // inclusive range via .after()/.before().
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
#[ignore = "scratch playground, not a real test — run explicitly with `cargo test --test scratch_playground -- --ignored`"]
fn scratch6_tiny() {
    // TINY, per std's own next_up()/next_down() docs: "the smallest
    // representable positive f32" — i.e. the smallest positive *subnormal*,
    // not f32::MIN_POSITIVE (which is the smallest *normal* value).
    let tiny = f32::from_bits(1);
    println!("TINY = {tiny:e} (bits: {:#010x})", tiny.to_bits());
    println!(
        "TINY == 0f32.next_up()? {}",
        tiny.to_bits() == 0f32.next_up().to_bits()
    );
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

// ---------------------------------------------------------------------
// Every function that uses `unsafe` (transmute or otherwise) under the
// covers. Grep for `unsafe` in src/float/{finite,total}.rs to see the
// full list this section is tracking:
//   - Finite::new_unchecked                    (unsafe fn — direct value-invariant escape hatch)
//   - Finite::try_new                          (safe fn, built on new_unchecked)
//   - Finite::from_primitive_slice_unchecked   (unsafe fn — transmute &[Primitive] -> &[Finite])
//   - Finite::from_primitive_slice             (safe fn, built on from_primitive_slice_unchecked)
//   - FiniteSliceExt::as_primitive_slice       (safe fn — transmute &[Finite] -> &[Primitive])
//   - Total::from_primitive_slice              (safe fn — transmute &[Primitive] -> &[Total])
//   - TotalSliceExt::as_primitive_slice        (safe fn — transmute &[Total] -> &[Primitive])
// ---------------------------------------------------------------------

#[test]
#[ignore = "scratch playground, not a real test — run explicitly with `cargo test --test scratch_playground -- --ignored`"]
fn scratch8_new_unchecked_correct_use() {
    // `Finite::new_unchecked` is the unsafe building block every safe
    // constructor is defined in terms of. Used correctly (finite, and zero
    // already canonicalized to +0.0), it behaves exactly like `new`.
    let via_unchecked = unsafe { FiniteF64::new_unchecked(42.0) };
    let via_safe = FiniteF64::new(42.0);
    println!("new_unchecked(42.0) = {via_unchecked:?}, new(42.0) = {via_safe:?}");
    assert_eq!(via_unchecked, via_safe);
}

#[test]
#[ignore = "scratch playground, not a real test — run explicitly with `cargo test --test scratch_playground -- --ignored`"]
fn scratch9_new_unchecked_misuse() {
    // Violate the documented precondition (pass -0.0, which must already be
    // canonicalized to +0.0). Per the safety doc, this is "nonsense, but
    // not unsafe": no UB, but the type's invariant (no -0.0) is now broken,
    // and that breaks the `PartialEq`/`total_cmp`-based ordering that the
    // rest of the crate assumes holds for every live `Finite` value.
    let broken = unsafe { FiniteF64::new_unchecked(-0.0) };
    println!(
        "new_unchecked(-0.0) = {broken:?}, is_sign_negative = {}",
        broken.into_inner().is_sign_negative()
    );
    assert!(broken.into_inner().is_sign_negative());

    // Compare: the safe constructor refuses to let this happen.
    println!("for reference, new(-0.0) = {:?}", FiniteF64::new(-0.0));
    assert!(!FiniteF64::new(-0.0).into_inner().is_sign_negative());
}

#[test]
#[ignore = "scratch playground, not a real test — run explicitly with `cargo test --test scratch_playground -- --ignored`"]
fn scratch10_try_new() {
    // `try_new` is safe because it independently establishes finiteness before
    // calling `new_unchecked` internally.
    assert_eq!(FiniteF64::try_new(1.5), Some(FiniteF64::new(1.5)));
    assert_eq!(FiniteF64::try_new(f64::NAN), None);
    assert_eq!(FiniteF64::try_new(f64::INFINITY), None);
}

#[test]
#[ignore = "scratch playground, not a real test — run explicitly with `cargo test --test scratch_playground -- --ignored`"]
fn scratch11_finite_slice_roundtrip() {
    // `Finite::from_primitive_slice` validates every element (O(n)) then calls
    // the unsafe, zero-copy `from_primitive_slice_unchecked` (a `mem::transmute`
    // of `&[f64]` to `&[Finite<f64>]`, sound because `Finite` is `#[repr(transparent)]`).
    let primitives = [1.0, 2.0, 3.0];
    let finites: &[FiniteF64] = FiniteF64::from_primitive_slice(&primitives);
    println!("FiniteF64::from_primitive_slice({primitives:?}) = {finites:?}");
    assert_eq!(
        finites,
        [
            FiniteF64::new(1.0),
            FiniteF64::new(2.0),
            FiniteF64::new(3.0)
        ]
    );

    // Same pointer/length — it's a view, not a copy.
    assert_eq!(finites.as_ptr().cast::<f64>(), primitives.as_ptr());
    assert_eq!(finites.len(), primitives.len());

    // The reverse view, `FiniteSliceExt::as_primitive_slice`, is also just a transmute.
    let back: &[f64] = finites.as_primitive_slice();
    println!("finites.as_primitive_slice() = {back:?}");
    assert_eq!(back, primitives);
    assert_eq!(back.as_ptr(), finites.as_ptr().cast::<f64>());
}

#[test]
#[ignore = "scratch playground, not a real test — run explicitly with `cargo test --test scratch_playground -- --ignored`"]
#[should_panic(expected = "Finite type requires finite")]
fn scratch12_finite_slice_rejects_bad_input() {
    // `Finite::from_primitive_slice` validates before transmuting; NaN in the
    // input panics rather than silently producing a `Finite` slice with a broken value.
    let _ = FiniteF64::from_primitive_slice(&[1.0, f64::NAN, 3.0]);
}

#[test]
#[ignore = "scratch playground, not a real test — run explicitly with `cargo test --test scratch_playground -- --ignored`"]
fn scratch13_slice_unchecked_misuse() {
    // `from_primitive_slice_unchecked` skips the validation `from_primitive_slice`
    // does. Feeding it NaN (violating the safety precondition) is, again,
    // "nonsense but not unsafe": no UB, but every `Finite` value in the
    // returned slice is now a lie -- `is_finite()`-style reasoning downstream
    // can no longer be trusted for this slice.
    let primitives = [1.0, f64::NAN, 3.0];
    let finites: &[FiniteF64] = unsafe { FiniteF64::from_primitive_slice_unchecked(&primitives) };
    println!("from_primitive_slice_unchecked({primitives:?}) = {finites:?}");
    assert!(finites[1].into_inner().is_nan());
}

#[test]
#[ignore = "scratch playground, not a real test — run explicitly with `cargo test --test scratch_playground -- --ignored`"]
fn scratch14_total_slice_roundtrip() {
    // `Total::from_primitive_slice` and `TotalSliceExt::as_primitive_slice`
    // are both *safe* functions (unlike their `Finite` counterparts) because
    // `Total` has no value invariant to violate -- every bit pattern of the
    // primitive, including NaN and -0.0, is a legal `Total` value. The
    // transmute is still there under the hood, just without an
    // unchecked/checked split.
    let primitives = [1.0f32, f32::NAN, -0.0, f32::INFINITY];
    let totals: &[TotalF32] = TotalF32::from_primitive_slice(&primitives);
    println!("TotalF32::from_primitive_slice({primitives:?}) = {totals:?}");
    assert_eq!(totals.as_ptr().cast::<f32>(), primitives.as_ptr());

    let back: &[f32] = totals.as_primitive_slice();
    println!("totals.as_primitive_slice() = {back:?}");
    assert_eq!(back.as_ptr(), totals.as_ptr().cast::<f32>());
    // NaN survives the round trip bit-for-bit (transmute, not a copy/parse).
    assert!(back[1].is_nan());
    assert!(back[2].is_sign_negative() && back[2] == 0.0);
}

#[test]
#[ignore = "scratch playground, not a real test — run explicitly with `cargo test --test scratch_playground -- --ignored`"]
fn scratch15_merge_float_intervals() {
    // Turn a bunch of overlapping and adjacent inclusive primitive ranges into a
    // coalesced set of disjoint inclusive ranges.
    let overlapping_intervals = vec![
        (11.0, 13.0),
        (1.0, 3.0),
        (30.0, 31.0),
        (5.5, 9.0),
        (-4.0, 2.0),
        (15.0, 15.0),
        (7.0, 7.5),
        (21.0, 28.0),
        (2.5, 6.0),
        (19.0, 22.0),
    ];

    let disjoint_intervals: Vec<_> = overlapping_intervals
        .into_iter()
        .map(|(s, e)| FiniteF64::from_primitive_range(s..=e)) // Wrap each range
        .collect::<RangeSetBlaze<_>>() // Coalesce into disjoint ranges
        .ranges() // Convert back
        .map(FiniteRangeExt::into_primitive_inner)
        .collect();

    assert_eq!(
        disjoint_intervals,
        vec![
            (-4.0, 9.0),
            (11.0, 13.0),
            (15.0, 15.0),
            (19.0, 28.0),
            (30.0, 31.0)
        ]
    );
}
