//! Tests

#![cfg(test)]
#![cfg(feature = "total_float_experimental")]

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

#[cfg(feature = "total_float_nightly_experimental")]
const BIG_ZERO: UIntPlusOne<u128> = UIntPlusOne::<u128>::UInt(0);

#[test]
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

#[test]
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
