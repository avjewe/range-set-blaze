//! Tests

#![cfg(test)]
#![cfg(feature = "total_float_experimental")]
#![cfg_attr(feature = "total_float_nightly_experimental", feature(f16))]
#![cfg_attr(feature = "total_float_nightly_experimental", feature(f128))]

use num_traits::identities::One;
#[cfg(feature = "total_float_nightly_experimental")]
use range_set_blaze::UIntPlusOne;
#[cfg(feature = "total_float_nightly_experimental")]
use range_set_blaze::finite::{ff16, ff128};
use range_set_blaze::finite::{ff32, ff64};
#[cfg(feature = "total_float_nightly_experimental")]
use range_set_blaze::total::{tf16, tf128};
use range_set_blaze::total::{tf32, tf64};
#[cfg(feature = "total_float_nightly_experimental")]
use range_set_blaze::{FiniteF16, FiniteF128, TotalF16, TotalF128};
use range_set_blaze::{
    FiniteF32, FiniteF64, Integer, RangeMapBlaze, RangeSetBlaze, TotalF32, TotalF64,
};
use syntactic_for::syntactic_for;
use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

#[cfg(feature = "total_float_nightly_experimental")]
const BIG_ZERO: UIntPlusOne<u128> = UIntPlusOne::<u128>::UInt(0);

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn map_complement0() {
    assert!(0.0 == -0.0);
    assert_eq!(0.0, -0.0);
    assert_ne!(tf64(0.0), tf64(-0.0));
    assert_eq!(ff64(0.0), ff64(-0.0));
    assert_ne!(tf32(0.0), tf32(-0.0));
    assert_eq!(ff32(0.0), ff32(-0.0));

    let empty = RangeMapBlaze::<TotalF64, u8>::new();
    assert_eq!(empty.len(), 0);
    let full = !&empty;
    assert_eq!(full.len(), i128::from(u64::MAX) + 1);
    let empty = !&full;
    assert_eq!(empty.len(), 0);

    let empty = RangeMapBlaze::<TotalF32, u8>::new();
    assert_eq!(empty.len(), 0);
    let full = !&empty;
    assert_eq!(full.len(), i64::from(u32::MAX) + 1);
    let empty = !&full;
    assert_eq!(empty.len(), 0);

    let empty = RangeMapBlaze::<FiniteF64, u8>::new();
    assert_eq!(empty.len(), 0);
    let full = !&empty;
    assert_eq!(full.len(), FiniteF64::MAX_SIZE);
    let empty = !&full;
    assert_eq!(empty.len(), 0);

    let empty = RangeMapBlaze::<FiniteF32, u8>::new();
    assert_eq!(empty.len(), 0);
    let full = !&empty;
    assert_eq!(full.len(), FiniteF32::MAX_SIZE);
    let empty = !&full;
    assert_eq!(empty.len(), 0);
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg(feature = "total_float_nightly_experimental")]
fn map_complement0_nightly() {
    assert_ne!(tf16(0.0), tf16(-0.0));
    assert_eq!(ff16(0.0), ff16(-0.0));
    assert_ne!(tf128(0.0), tf128(-0.0));
    assert_eq!(ff128(0.0), ff128(-0.0));

    let empty = RangeMapBlaze::<TotalF128, u8>::new();
    assert_eq!(empty.len(), BIG_ZERO);
    let full = !&empty;
    assert_eq!(full.len(), UIntPlusOne::<u128>::MaxPlusOne);
    let empty = !&full;
    assert_eq!(empty.len(), BIG_ZERO);

    let empty = RangeMapBlaze::<TotalF16, u8>::new();
    assert_eq!(empty.len(), 0);
    let full = !&empty;
    assert_eq!(full.len(), i32::from(u16::MAX) + 1);
    let empty = !&full;
    assert_eq!(empty.len(), 0);

    let empty = RangeMapBlaze::<FiniteF128, u8>::new();
    assert_eq!(empty.len(), 0);
    let full = !&empty;
    assert_eq!(full.len(), FiniteF128::MAX_SIZE);
    let empty = !&full;
    assert_eq!(empty.len(), 0);

    let empty = RangeMapBlaze::<FiniteF16, u8>::new();
    assert_eq!(empty.len(), 0);
    let full = !&empty;
    assert_eq!(full.len(), FiniteF16::MAX_SIZE);
    let empty = !&full;
    assert_eq!(empty.len(), 0);
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn set_complement0() {
    let empty = RangeSetBlaze::<TotalF64>::new();
    assert_eq!(empty.len(), 0);
    let full = !&empty;
    assert_eq!(full.len(), i128::from(u64::MAX) + 1);
    let empty = !&full;
    assert_eq!(empty.len(), 0);

    let empty = RangeSetBlaze::<TotalF32>::new();
    assert_eq!(empty.len(), 0);
    let full = !&empty;
    assert_eq!(full.len(), i64::from(u32::MAX) + 1);
    let empty = !&full;
    assert_eq!(empty.len(), 0);

    let empty = RangeSetBlaze::<FiniteF64>::new();
    assert_eq!(empty.len(), 0);
    let full = !&empty;
    assert_eq!(full.len(), FiniteF64::MAX_SIZE);
    let empty = !&full;
    assert_eq!(empty.len(), 0);

    let empty = RangeSetBlaze::<FiniteF32>::new();
    assert_eq!(empty.len(), 0);
    let full = !&empty;
    assert_eq!(full.len(), FiniteF32::MAX_SIZE);
    let empty = !&full;
    assert_eq!(empty.len(), 0);
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg(feature = "total_float_nightly_experimental")]
fn set_complement_nightly() {
    let empty = RangeSetBlaze::<TotalF128>::new();
    assert_eq!(empty.len(), BIG_ZERO);
    let full = !&empty;
    assert_eq!(full.len(), UIntPlusOne::<u128>::MaxPlusOne);
    let empty = !&full;
    assert_eq!(empty.len(), BIG_ZERO);

    let empty = RangeSetBlaze::<TotalF16>::new();
    assert_eq!(empty.len(), 0);
    let full = !&empty;
    assert_eq!(full.len(), i32::from(u16::MAX) + 1);
    let empty = !&full;
    assert_eq!(empty.len(), 0);

    let empty = RangeSetBlaze::<FiniteF128>::new();
    assert_eq!(empty.len(), 0);
    let full = !&empty;
    assert_eq!(full.len(), FiniteF128::MAX_SIZE);
    let empty = !&full;
    assert_eq!(empty.len(), 0);

    let empty = RangeSetBlaze::<FiniteF16>::new();
    assert_eq!(empty.len(), 0);
    let full = !&empty;
    assert_eq!(full.len(), FiniteF16::MAX_SIZE);
    let empty = !&full;
    assert_eq!(empty.len(), 0);
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[allow(clippy::cognitive_complexity, clippy::float_cmp)]
fn integer_coverage() {
    syntactic_for! { ty in [TotalF32, TotalF64, FiniteF32, FiniteF64] {
        $(
            let len = <$ty as Integer>::SafeLen::one();
            let a = $ty::new(42.0);
            assert_eq!($ty::safe_len_to_f64_lossy(len), 1.0);
            assert_eq!($ty::inclusive_end_from_start(a,len), a);
            assert_eq!($ty::start_from_inclusive_end(a,len), a);
            assert_eq!($ty::f64_to_safe_len_lossy(1.0), len);

        )*
    }};
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg(feature = "total_float_nightly_experimental")]
#[allow(clippy::cognitive_complexity, clippy::float_cmp)]
fn integer_coverage_nightly() {
    syntactic_for! { ty in [TotalF16, TotalF128, FiniteF16, FiniteF128] {
        $(
            let len = <$ty as Integer>::SafeLen::one();
            let a = $ty::new(42.0);
            assert_eq!($ty::safe_len_to_f64_lossy(len), 1.0);
            assert_eq!($ty::inclusive_end_from_start(a,len), a);
            assert_eq!($ty::start_from_inclusive_end(a,len), a);
            assert_eq!($ty::f64_to_safe_len_lossy(1.0), len);

        )*
    }};
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
// I don't quite understand why clippy complains here
#[expect(clippy::from_iter_instead_of_collect)]
fn float_test() {
    let _ = RangeSetBlaze::<TotalF64>::new();
    let _ = RangeSetBlaze::<TotalF32>::new();

    let _ = RangeSetBlaze::from_iter([tf64(3.0)..=tf64(5.0)]);
    let _ = RangeSetBlaze::from_iter([tf32(3.0)..=tf32(5.0)]);
    let _ = RangeSetBlaze::from_iter([tf64(1.0), tf64(2.0), tf64(3.0)]);
    let _ = RangeSetBlaze::from_iter([tf32(1.0), tf32(2.0), tf32(3.0)]);

    let _ = RangeSetBlaze::from(tf64(3.0)..=tf64(5.0));
    let _ = RangeSetBlaze::from(tf32(3.0)..=tf32(5.0));

    let _ = RangeSetBlaze::from(TotalF64::range(3.0..=5.0));
    let _ = RangeSetBlaze::from(TotalF32::range(3.0..=5.0));

    let _ = RangeSetBlaze::from_iter(TotalF64::ranges([3.0..=5.0, 7.0..=9.0]));
    let _ = RangeSetBlaze::from_iter(TotalF32::ranges([3.0..=5.0, 7.0..=9.0]));

    let _ = RangeSetBlaze::from_iter(TotalF64::slice(&[1.0, 2.0, 3.0]));
    let _ = RangeSetBlaze::from_iter(TotalF32::slice(&[1.0, 2.0, 3.0]));
    let _ = RangeSetBlaze::from_iter(TotalF64::values([1.0, 2.0, 3.0]));
    let _ = RangeSetBlaze::from_iter(TotalF32::values([1.0, 2.0, 3.0]));

    let foo = RangeSetBlaze::from_iter(TotalF64::ranges([3.0..=5.0, 7.0..=9.0]));
    assert!(foo.contains(tf64(3.0)));
    assert!(foo.contains(tf64(5.0)));
    assert!(foo.contains(tf64(7.0)));
    assert!(foo.contains(tf64(9.0)));

    assert!(foo.contains(tf64(3.01)));
    assert!(foo.contains(tf64(4.99)));
    assert!(foo.contains(tf64(7.01)));
    assert!(foo.contains(tf64(8.99)));

    assert!(!foo.contains(tf64(2.99)));
    assert!(!foo.contains(tf64(5.01)));
    assert!(!foo.contains(tf64(6.99)));
    assert!(!foo.contains(tf64(9.01)));

    assert!(!foo.contains(tf64(3.0).prev()));
    assert!(!foo.contains(tf64(5.0).next()));
    assert!(!foo.contains(tf64(7.0).prev()));
    assert!(!foo.contains(tf64(9.0).next()));

    assert!(foo.contains(tf64(3.0).next()));
    assert!(foo.contains(tf64(5.0).prev()));
    assert!(foo.contains(tf64(7.0).next()));
    assert!(foo.contains(tf64(9.0).prev()));
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn test_inclusive() {
    syntactic_for! { ty in [TotalF32, TotalF64, FiniteF32, FiniteF64] {
        $(
    let a = <$ty>::min_value();
    let b = <$ty>::max_value();
    let len = <$ty>::safe_len(&(a..=b));
    assert_eq!(<$ty>::inclusive_end_from_start(a, len), b);
    assert_eq!(<$ty>::start_from_inclusive_end(b, len), a);
        )*
    }}
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg(feature = "total_float_nightly_experimental")]
fn test_inclusive_nightly() {
    syntactic_for! { ty in [TotalF16, TotalF128, FiniteF16, FiniteF128] {
        $(
            let a = <$ty>::min_value();
            let b = <$ty>::max_value();
            let len = <$ty>::safe_len(&(a..=b));
            assert_eq!(<$ty>::inclusive_end_from_start(a, len), b);
            assert_eq!(<$ty>::start_from_inclusive_end(b, len), a);
        )*
    }}
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn test_floats2() {
    syntactic_for! { ty in [TotalF32, TotalF64, FiniteF32, FiniteF64] {
        $(
            let mut a = $ty::range(0.0..=0.0);
            assert_eq!($ty::range_next_back(&mut a), Some($ty::new(0.0)));
            assert_eq!($ty::range_next(&mut a), None);

            let mut b = $ty::new(0.0);
            $ty::assign_sub_one(&mut b);
            assert_eq!(b, $ty::new(0.0).prev());
        )*
    }}
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg(feature = "total_float_nightly_experimental")]
fn test_floats2_nightly() {
    syntactic_for! { ty in [TotalF16, TotalF128, FiniteF16, FiniteF128] {
        $(
            let mut a = $ty::range(0.0..=0.0);
            assert_eq!($ty::range_next_back(&mut a), Some($ty::new(0.0)));
            assert_eq!($ty::range_next(&mut a), None);

            let mut b = $ty::new(0.0);
            $ty::assign_sub_one(&mut b);
            assert_eq!(b, $ty::new(0.0).prev());
        )*
    }}
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn test_floats() {
    let mut a = TotalF32::range(0.0..=0.0);
    assert_eq!(TotalF32::range_next_back(&mut a), Some(tf32(0.0)));
    assert_eq!(TotalF32::range_next(&mut a), None);

    let mut a = TotalF64::range(0.0..=0.0);
    assert_eq!(TotalF64::range_next_back(&mut a), Some(tf64(0.0)));
    assert_eq!(TotalF64::range_next(&mut a), None);

    let mut b = tf64(0.0);
    TotalF64::assign_sub_one(&mut b);
    assert_eq!(b, tf64(0.0).prev());

    let mut b = tf32(0.0);
    TotalF32::assign_sub_one(&mut b);
    assert_eq!(b, tf32(0.0).prev());
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg(feature = "total_float_nightly_experimental")]
fn test_floats_nightly() {
    let mut a = TotalF16::range(0.0..=0.0);
    assert_eq!(TotalF16::range_next_back(&mut a), Some(tf16(0.0)));
    assert_eq!(TotalF16::range_next(&mut a), None);

    let mut b = tf16(0.0);
    TotalF16::assign_sub_one(&mut b);
    assert_eq!(b, tf16(0.0).prev());

    let mut a = TotalF128::range(0.0..=0.0);
    assert_eq!(TotalF128::range_next_back(&mut a), Some(tf128(0.0)));
    assert_eq!(TotalF128::range_next(&mut a), None);

    let mut b = tf128(0.0);
    TotalF128::assign_sub_one(&mut b);
    assert_eq!(b, tf128(0.0).prev());
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn total_iterators() {
    syntactic_for! { ty in [TotalF32, TotalF64, FiniteF32, FiniteF64] {
        $(
            // MAX forward
            let set = RangeSetBlaze::from_iter([$ty::MAX..=$ty::MAX]);
            let mut iter = set.iter();
            assert_eq!(iter.next(), Some($ty::MAX));
            assert_eq!(iter.next(), None);

            // MAX reverse
            let set = RangeSetBlaze::from_iter([$ty::MAX..=$ty::MAX]);
            let mut iter = set.iter().rev();
            assert_eq!(iter.next(), Some($ty::MAX));
            assert_eq!(iter.next(), None);

            // MIN forward
            let set = RangeSetBlaze::from_iter([$ty::MIN..=$ty::MIN]);
            let mut iter = set.iter();
            assert_eq!(iter.next(), Some($ty::MIN));
            assert_eq!(iter.next(), None);

            // MIN reverse
            let set = RangeSetBlaze::from_iter([$ty::MIN..=$ty::MIN]);
            let mut iter = set.iter().rev();
            assert_eq!(iter.next(), Some($ty::MIN));
            assert_eq!(iter.next(), None);
        )*
    }}
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg(feature = "total_float_nightly_experimental")]
fn total_iterators_nightly() {
    syntactic_for! { ty in [TotalF16, TotalF128, FiniteF16, FiniteF128] {
        $(
            // MAX forward
            let set = RangeSetBlaze::from_iter([$ty::MAX..=$ty::MAX]);
            let mut iter = set.iter();
            assert_eq!(iter.next(), Some($ty::MAX));
            assert_eq!(iter.next(), None);

            // MAX reverse
            let set = RangeSetBlaze::from_iter([$ty::MAX..=$ty::MAX]);
            let mut iter = set.iter().rev();
            assert_eq!(iter.next(), Some($ty::MAX));
            assert_eq!(iter.next(), None);

            // MIN forward
            let set = RangeSetBlaze::from_iter([$ty::MIN..=$ty::MIN]);
            let mut iter = set.iter();
            assert_eq!(iter.next(), Some($ty::MIN));
            assert_eq!(iter.next(), None);

            // MIN reverse
            let set = RangeSetBlaze::from_iter([$ty::MIN..=$ty::MIN]);
            let mut iter = set.iter().rev();
            assert_eq!(iter.next(), Some($ty::MIN));
            assert_eq!(iter.next(), None);
        )*
    }}
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn total_complement() {
    syntactic_for! { ty in [TotalF32, TotalF64, FiniteF32, FiniteF64] {
        $(
            let set = RangeSetBlaze::from_iter([$ty::MAX..=$ty::MAX]);
            assert!(set.contains($ty::MAX));
            assert!(!set.contains($ty::MAX.prev()));
            assert_eq!(set.len(), 1);
            let set = !set;
            assert!(!set.contains($ty::MAX));
            assert!(set.contains($ty::MAX.prev()));
            assert_eq!(set.len(), $ty::MAX_SIZE - 1);

            let set = RangeSetBlaze::from_iter([$ty::MIN..=$ty::MIN]);
            assert!(set.contains($ty::MIN));
            assert!(!set.contains($ty::MIN.next()));
            assert_eq!(set.len(), 1);
            let set = !set;
            assert!(!set.contains($ty::MIN));
            assert!(set.contains($ty::MIN.next()));
            assert_eq!(set.len(), $ty::MAX_SIZE - 1);

            let set = RangeSetBlaze::from_iter([$ty::MIN..=$ty::MIN.next()]);
            assert!(set.contains($ty::MIN));
            assert!(set.contains($ty::MIN.next()));
            assert!(!set.contains($ty::MIN.next().next()));
            assert_eq!(set.len(), 2);
            let set = !set;
            assert!(!set.contains($ty::MIN));
            assert!(!set.contains($ty::MIN.next()));
            assert!(set.contains($ty::MIN.next().next()));
            assert_eq!(set.len(), $ty::MAX_SIZE - 2);
        )*
    }}
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg(feature = "total_float_nightly_experimental")]
fn total_complement_nightly() {
    // Total128::SafeLen is UIntPlusOne, which doesn't work smoothly here, so it's a separate test below
    syntactic_for! { ty in [TotalF16, /*TotalF128,*/ FiniteF16, FiniteF128] {
        $(
            let set = RangeSetBlaze::from_iter([$ty::MAX..=$ty::MAX]);
            assert!(set.contains($ty::MAX));
            assert!(!set.contains($ty::MAX.prev()));
            assert_eq!(set.len(), 1);
            let set = !set;
            assert!(!set.contains($ty::MAX));
            assert!(set.contains($ty::MAX.prev()));
            assert_eq!(set.len(), $ty::MAX_SIZE - 1);

            let set = RangeSetBlaze::from_iter([$ty::MIN..=$ty::MIN]);
            assert!(set.contains($ty::MIN));
            assert!(!set.contains($ty::MIN.next()));
            assert_eq!(set.len(), 1);
            let set = !set;
            assert!(!set.contains($ty::MIN));
            assert!(set.contains($ty::MIN.next()));
            assert_eq!(set.len(), $ty::MAX_SIZE - 1);

            let set = RangeSetBlaze::from_iter([$ty::MIN..=$ty::MIN.next()]);
            assert!(set.contains($ty::MIN));
            assert!(set.contains($ty::MIN.next()));
            assert!(!set.contains($ty::MIN.next().next()));
            assert_eq!(set.len(), 2);
            let set = !set;
            assert!(!set.contains($ty::MIN));
            assert!(!set.contains($ty::MIN.next()));
            assert!(set.contains($ty::MIN.next().next()));
            assert_eq!(set.len(), $ty::MAX_SIZE - 2);
        )*
    }}
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg(feature = "total_float_nightly_experimental")]
fn total_complement_total128() {
    let set = RangeSetBlaze::from_iter([TotalF128::MAX..=TotalF128::MAX]);
    assert!(set.contains(TotalF128::MAX));
    assert!(!set.contains(TotalF128::MAX.prev()));
    assert_eq!(set.len(), UIntPlusOne::<u128>::UInt(1));
    let set = !set;
    assert!(!set.contains(TotalF128::MAX));
    assert!(set.contains(TotalF128::MAX.prev()));
    assert_eq!(set.len(), UIntPlusOne::<u128>::UInt(u128::MAX));

    let set = RangeSetBlaze::from_iter([TotalF128::MIN..=TotalF128::MIN]);
    assert!(set.contains(TotalF128::MIN));
    assert!(!set.contains(TotalF128::MIN.next()));
    assert_eq!(set.len(), UIntPlusOne::<u128>::UInt(1));
    let set = !set;
    assert!(!set.contains(TotalF128::MIN));
    assert!(set.contains(TotalF128::MIN.next()));
    assert_eq!(set.len(), UIntPlusOne::<u128>::UInt(u128::MAX));

    let set = RangeSetBlaze::from_iter([TotalF128::MIN..=TotalF128::MIN.next()]);
    assert!(set.contains(TotalF128::MIN));
    assert!(set.contains(TotalF128::MIN.next()));
    assert!(!set.contains(TotalF128::MIN.next().next()));
    assert_eq!(set.len(), UIntPlusOne::<u128>::UInt(2));
    let set = !set;
    assert!(!set.contains(TotalF128::MIN));
    assert!(!set.contains(TotalF128::MIN.next()));
    assert!(set.contains(TotalF128::MIN.next().next()));
    assert_eq!(set.len(), UIntPlusOne::<u128>::UInt(u128::MAX - 1));
}

// ============================================================================
// KNOWN BUGS (TDD red step): the tests below currently FAIL. They document
// real invariant violations, not test-writing mistakes. `FiniteF64`'s own doc
// comment (see `FiniteF64` / `Finite<T>`) promises values "excluding NaN,
// -0.0, and infinities," and `new()`/`try_new()` enforce that via
// `T::is_finite` + `T::normalize`. But `range()`, `ranges()`, `values()`, and
// `slice()` are convenience/zero-copy constructors that build `Self` directly
// (or `transmute` for `slice()`), bypassing both the finiteness check and the
// -0.0 normalization. Left as failing tests on purpose so this is visible in
// CI until someone (author or reviewer) decides how to fix it.
// ============================================================================

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[should_panic(expected = "Finite type requires a finite value")]
fn finite_range_rejects_nan_start() {
    let _ = FiniteF64::range(f64::NAN..=1.0);
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[should_panic(expected = "Finite type requires a finite value")]
fn finite_range_rejects_infinite_end() {
    let _ = FiniteF64::range(1.0..=f64::INFINITY);
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[should_panic(expected = "Finite type requires a finite value")]
fn finite_values_rejects_nan() {
    // Important: values() is lazy, so force iteration.
    let _ = FiniteF64::values([1.0, f64::NAN]).collect::<Vec<_>>();
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn finite_values_normalizes_negative_zero() {
    let value = FiniteF64::values([-0.0]).next().unwrap();

    assert_eq!(value, FiniteF64::new(0.0));
    assert_eq!(value.into_inner().to_bits(), 0.0f64.to_bits());
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn finite_slice_can_create_invalid_finite_value() {
    let values = FiniteF64::slice(&[f64::NAN]);

    assert!(
        !values[0].into_inner().is_nan(),
        "safe FiniteF64::slice allowed a NaN-backed FiniteF64"
    );
}

// The bypass logic in `range`/`values`/`slice` is shared generic code (one
// impl block in `finite.rs`, monomorphized per type) — the tests above
// already prove the bug there once. These f32/f128 variants aren't re-testing
// that shared logic; they sanity-check that each type's own `FiniteFloat::
// is_finite`/`normalize` leaf impl (which live separately per type in
// `finite_float.rs`) behaves the same way once a fix routes through them.

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[should_panic(expected = "Finite type requires a finite value")]
fn finite_range_rejects_nan_start_f32() {
    let _ = FiniteF32::range(f32::NAN..=1.0);
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn finite_values_normalizes_negative_zero_f32() {
    let value = FiniteF32::values([-0.0]).next().unwrap();

    assert_eq!(value, FiniteF32::new(0.0));
    assert_eq!(value.into_inner().to_bits(), 0.0f32.to_bits());
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn finite_slice_can_create_invalid_finite_value_f32() {
    let values = FiniteF32::slice(&[f32::NAN]);

    assert!(
        !values[0].into_inner().is_nan(),
        "safe FiniteF32::slice allowed a NaN-backed FiniteF32"
    );
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg(feature = "total_float_nightly_experimental")]
#[should_panic(expected = "Finite type requires a finite value")]
fn finite_range_rejects_nan_start_f128() {
    let _ = FiniteF128::range(f128::NAN..=1.0);
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg(feature = "total_float_nightly_experimental")]
fn finite_values_normalizes_negative_zero_f128() {
    let value = FiniteF128::values([-0.0]).next().unwrap();

    assert_eq!(value, FiniteF128::new(0.0));
    assert_eq!(value.into_inner().to_bits(), 0.0f128.to_bits());
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg(feature = "total_float_nightly_experimental")]
fn finite_slice_can_create_invalid_finite_value_f128() {
    let values = FiniteF128::slice(&[f128::NAN]);

    assert!(
        !values[0].into_inner().is_nan(),
        "safe FiniteF128::slice allowed a NaN-backed FiniteF128"
    );
}

// ============================================================================
// Why didn't the exhaustive `full_16` test above already catch this?
//
// `full_16` walks the *already-valid* total-order domain: it starts at
// `$ty::MIN` (a finite extreme) and steps forward only via `.next()`, whose
// own logic already knows to skip the -0.0 ordered slot and never produces
// NaN. It exhaustively checks internal self-consistency of that walk
// (safe_len, inclusive_end_from_start, next/prev symmetry) — it never routes
// arbitrary/adversarial *input* bit patterns (NaN, -0.0, +/-infinity) through
// the other public entry points (`values`, `range`, `slice`), which is
// exactly where the bug lives. Below is the same exhaustive-over-all-bits
// idea as `full_16`, but aimed at the constructors instead of the walk.
// ============================================================================

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg(feature = "total_float_nightly_experimental")]
fn finite_f16_exhaustive_bit_patterns_via_bypass_constructors() {
    let mut failures = Vec::new();

    for bits in 0..=u16::MAX {
        let x = f16::from_bits(bits);

        let via_values = FiniteF16::values([x]).next().unwrap();
        if !via_values.into_inner().is_finite() {
            failures.push(format!(
                "values(): bits {bits:#06x} ({x}) produced non-finite FiniteF16"
            ));
        } else if via_values.into_inner() == 0.0 && via_values.into_inner().is_sign_negative() {
            failures.push(format!(
                "values(): bits {bits:#06x} produced -0.0-backed FiniteF16 (should normalize to +0.0)"
            ));
        }

        let via_range = *FiniteF16::range(x..=x).start();
        if !via_range.into_inner().is_finite() {
            failures.push(format!(
                "range(): bits {bits:#06x} ({x}) produced non-finite FiniteF16"
            ));
        }

        let via_slice = FiniteF16::slice(core::slice::from_ref(&x))[0];
        if !via_slice.into_inner().is_finite() {
            failures.push(format!(
                "slice(): bits {bits:#06x} ({x}) produced non-finite FiniteF16"
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "{} invariant violations found across all 65536 f16 bit patterns; first 10:\n{}",
        failures.len(),
        failures[..failures.len().min(10)].join("\n")
    );
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg(feature = "total_float_nightly_experimental")]
fn full_16() {
    syntactic_for! { ty in [TotalF16,  FiniteF16] {
        $(
            let mut x = $ty::MIN;
            let mut count : <$ty as Integer>::SafeLen = 0;
            loop {
                assert!($ty::MIN <= x && x <= $ty::MAX);
                assert_eq!($ty::safe_len(&($ty::MIN..=x)), count + 1);
                assert_eq!($ty::safe_len(&(x..=$ty::MAX)), $ty::MAX_SIZE - count );
                assert_eq!($ty::inclusive_end_from_start(x, $ty::MAX_SIZE - count), $ty::MAX);
                assert_eq!($ty::start_from_inclusive_end($ty::MAX, $ty::MAX_SIZE - count), x);
                assert_eq!($ty::inclusive_end_from_start($ty::MIN, count+1), x);
                assert_eq!($ty::start_from_inclusive_end(x, count+1), $ty::MIN);
                if x != $ty::MIN  {
                    assert_eq!(x.prev().next(), x);
                    assert!(x.prev() < x);
                    assert_eq!($ty::safe_len(&(x.prev()..=x)), 2);
                    assert_eq!($ty::start_from_inclusive_end(x, 2), x.prev());
                    assert!(x.prev() < x);
                    assert!(x > x.prev());
                    assert!(x == x);
                }
                if x != $ty::MAX {
                    assert_eq!(x.next().prev(), x);
                    assert!(x.next() > x);
                    assert_eq!($ty::safe_len(&(x..=x.next())), 2);
                    assert_eq!($ty::inclusive_end_from_start(x,2), x.next());
                    assert!(x.next() > x);
                    assert!(x < x.next());
                }
                if x == $ty::MAX {
                    break;
                }
                x = x.next();
                count += 1;
            }
        )*
    }}
}

// ============================================================================
// KNOWN BUG (TDD red step): `Finite<T>`'s doc comment (see `Finite<T>` /
// `FiniteF64` above) promises every value it holds is "excluding NaN, -0.0,
// and infinities." `try_new`/`new` enforce that. But `from_ordered`
// (`Finite::from_ordered` in `finite.rs`, backed by `FiniteFloat::
// from_ordered` in `finite_float.rs`) is a *public*, infallible round-trip
// helper that accepts an arbitrary `Ordered` value with no validation at
// all -- not even a `debug_assert`. Any `Ordered` value outside
// `MIN_ORDERED..=MAX_ORDERED`, or equal to the ordered slot for -0.0,
// silently produces a `Finite` value wrapping NaN, +/-infinity, or -0.0.
// Left failing on purpose, same as the `values`/`range`/`slice` bugs above,
// until someone decides how to fix it (validate-and-panic, return a
// `Result`, or document/rename to make the danger unmissable).
// ============================================================================

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg(feature = "total_float_nightly_experimental")]
fn finite_f16_exhaustive_from_ordered_rejects_out_of_domain() {
    // Exhaustive: f16's `Ordered` type is `i16`, so there are only 65536
    // possible inputs -- walk every single one and record every input that
    // breaks `FiniteF16`'s finite/-0.0 invariant.
    let mut failures = Vec::new();

    for ordered in i16::MIN..=i16::MAX {
        let value = FiniteF16::from_ordered(ordered);
        let inner = value.into_inner();
        let is_negative_zero = inner == 0.0 && inner.is_sign_negative();

        if !inner.is_finite() || is_negative_zero {
            failures.push(format!(
                "from_ordered({ordered}) produced invalid FiniteF16 {inner:?} (finite: {}, is -0.0: {is_negative_zero})",
                inner.is_finite()
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "{} of 65536 ordered i16 values broke FiniteF16's finite/-0.0 invariant via from_ordered; first 10:\n{}",
        failures.len(),
        failures[..failures.len().min(10)].join("\n")
    );
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn finite_f32_from_ordered_accepts_nan_ordered_bug() {
    // Same bug as the f16 exhaustive test above, but a single concrete
    // example for f32 (whose Ordered domain, i32, is too large to walk
    // exhaustively here). `TotalF32` shares the exact same `Ordered`
    // encoding as `FiniteF32` and has no finiteness restriction, so we can
    // use it to manufacture "the Ordered position of f32::NAN" and feed it
    // straight into `FiniteF32::from_ordered`.
    let nan_ordered = TotalF32::new(f32::NAN).to_ordered();

    let value = FiniteF32::from_ordered(nan_ordered);

    assert!(
        !value.into_inner().is_nan(),
        "BUG: FiniteF32::from_ordered({nan_ordered}) produced a NaN-backed FiniteF32: {:?}",
        value.into_inner()
    );
}
