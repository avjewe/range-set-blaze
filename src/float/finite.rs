//! Finite is a floating point type, suitable for use in ranges. Only finite values are valid.
//!
//! Ordering and other semantics are as per normal floating point comparisons.
//!
//! Enable with `total_float_experimental` (stable, `FiniteF32`/`FiniteF64`) and
//! `total_float_nightly_experimental` (nightly, adds `FiniteF16`/`FiniteF128`).
//!```
//! use range_set_blaze::{RangeSetBlaze, FiniteF64, FiniteF32};
//! let set = RangeSetBlaze::from_iter([FiniteF64::new(3.0)..=FiniteF64::new(5.0)]);
//! assert!(set.contains(FiniteF64::new(3.1)));
//! assert!(!set.contains(FiniteF64::new(2.9)));
//!
//! let set = RangeSetBlaze::from(FiniteF64::range(3.0..=5.0));
//! assert!(set.contains(FiniteF64::new(4.9)));
//! assert!(!set.contains(FiniteF64::new(5.1)));
//!
//! let set = RangeSetBlaze::from_iter(FiniteF32::ranges([3.0..=5.0, 7.0..=9.0]));
//! assert!(set.contains(FiniteF32::new(4.0)));
//! assert!(!set.contains(FiniteF32::new(6.0)));
//!```

use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
use core::ops::RangeInclusive;

use super::finite_float::FiniteFloat;
use core::fmt::Debug;
use num_traits::One;
use num_traits::ops::wrapping::{WrappingAdd, WrappingSub};

#[cfg(feature = "from_slice")]
use crate::RangeSetBlaze;

/// Total ordered f64, excluding NaN, -0.0, and infinities.
pub type FiniteF64 = Finite<f64>;
/// Total ordered f32, excluding NaN, -0.0, and infinities.
pub type FiniteF32 = Finite<f32>;
/// Total ordered f16, excluding NaN, -0.0, and infinities.
#[cfg(feature = "total_float_nightly_experimental")]
pub type FiniteF16 = Finite<f16>;
/// Total ordered f128, excluding NaN, -0.0, and infinities.
#[cfg(feature = "total_float_nightly_experimental")]
pub type FiniteF128 = Finite<f128>;

/// Construct a [`FiniteF64`] from an `f64`. Shorthand for [`FiniteF64::new`]
#[must_use]
pub fn ff64(x: f64) -> FiniteF64 {
    FiniteF64::new(x)
}

/// Construct a [`FiniteF32`] from an `f32`. Shorthand for [`FiniteF32::new`]
#[must_use]
pub fn ff32(x: f32) -> FiniteF32 {
    FiniteF32::new(x)
}

/// Construct a [`FiniteF16`] from an `f16`. Shorthand for [`FiniteF16::new`]
#[cfg(feature = "total_float_nightly_experimental")]
#[must_use]
pub fn ff16(x: f16) -> FiniteF16 {
    FiniteF16::new(x)
}

/// Construct a [`FiniteF128`] from an `f128`. Shorthand for [`FiniteF128::new`]
#[cfg(feature = "total_float_nightly_experimental")]
#[must_use]
pub fn ff128(x: f128) -> FiniteF128 {
    FiniteF128::new(x)
}

/// Experimental: A transparent wrapper around [`f64`] and friends with total ordering.
///
/// Comparison, equality, and hashing all agree with `total_cmp`.
///
/// # Enabling
///
/// This type is experimental and must be enabled with the `total_float_experimental` feature.
/// ```bash
/// cargo add range-set-blaze --features "total_float_experimental"
/// ```
/// That provides the `Finite32` and `Finite64` types.
///
/// If you're building with nightly, you can instead use the `total_float_nightly_experimental` feature.
/// ```bash
/// cargo add range-set-blaze --features "total_float_nightly_experimental"
/// ```
/// To also use the `Finite16` and `Finite128` types.
#[repr(transparent)]
#[derive(Copy, Clone, Default, Debug)]
pub struct Finite<T: FiniteFloat>(T::Primitive);

impl<T: FiniteFloat> Finite<T> {
    /// The minimum value that can be represented by the type.\
    /// Maps directly to `crate::Integer::min_value()`
    ///
    /// # Examples
    /// ```
    /// use range_set_blaze::FiniteF64;
    ///
    /// assert_eq!(FiniteF64::MIN, FiniteF64::new(f64::MIN));
    /// ```
    pub const MIN: Self = Self(T::MIN);

    /// The maximum value that can be represented by the type.\
    /// Maps directly to [`crate::Integer::max_value()`]
    ///
    /// # Examples
    /// ```
    /// use range_set_blaze::FiniteF64;
    ///
    /// assert_eq!(FiniteF64::MAX, FiniteF64::new(f64::MAX));
    /// ```
    pub const MAX: Self = Self(T::MAX);

    /// The maximum possible size of a range, i.e. the size if `[MIN..=MAX]`
    /// For Finite types, this is a strange number, because there are a lot of NAN values.
    ///
    /// # Examples
    /// ```
    /// use range_set_blaze::FiniteF32;
    ///
    /// assert_eq!(FiniteF32::MAX_SIZE, 0xFF00_0000_u32 - 1);
    /// ```
    pub const MAX_SIZE: T::SafeLen = T::MAX_SIZE;

    /// Creates a new [`Finite`] from a primitive float.
    /// Only finite values are legal
    ///
    /// # Examples
    /// ```
    /// use range_set_blaze::FiniteF64;
    ///
    /// let _ = FiniteF64::new(1.0);
    /// ```
    /// # Panics
    ///
    /// Panics if `x.is_finite()` returns false
    #[must_use]
    pub fn new(x: T::Primitive) -> Self {
        Self::try_new(x).expect("Finite type requires a finite value")
    }

    /// Creates a new [`Finite`] from a primitive float.
    ///
    /// Returns `None` if the float is not finite (NaN or infinity).
    ///
    /// # Examples
    /// ```
    /// use range_set_blaze::FiniteF64;
    ///
    /// assert_eq!(FiniteF64::try_new(1.0), Some(FiniteF64::new(1.0)));
    /// assert_eq!(FiniteF64::try_new(f64::NAN), None);
    /// ```
    #[must_use]
    pub fn try_new(x: T::Primitive) -> Option<Self> {
        T::is_finite(x).then(|| Self(T::normalize(x)))
    }

    /// Computes `self + (b - 1)` where `b` is of type `SafeLen`.
    ///
    /// # Precondition
    /// `b` must be small enough that the result stays within range for `T`. This is
    /// checked with `debug_assert!` and is *not* checked in release builds, where
    /// violating it produces an unspecified (nonsense, but not unsafe) result rather
    /// than a panic. Callers are expected to only ever pass a `b` that satisfies this.
    #[must_use]
    pub fn inclusive_end_from_start(self, b: T::SafeLen) -> Self {
        Self(T::inclusive_end_from_start(self.0, b))
    }

    /// Computes `self - (b - 1)` where `b` is of type `SafeLen`.
    ///
    /// # Precondition
    /// `b` must be small enough that the result stays within range for `T`. This is
    /// checked with `debug_assert!` and is *not* checked in release builds, where
    /// violating it produces an unspecified (nonsense, but not unsafe) result rather
    /// than a panic. Callers are expected to only ever pass a `b` that satisfies this.
    #[must_use]
    pub fn start_from_inclusive_end(self, b: T::SafeLen) -> Self {
        Self(T::start_from_inclusive_end(self.0, b))
    }

    /// Returns the wrapped value.
    ///
    /// # Examples
    /// ```
    /// use range_set_blaze::FiniteF64;
    ///
    /// assert_eq!(FiniteF64::new(42.0).into_inner(), 42.0);
    /// ```
    #[must_use]
    pub const fn into_inner(self) -> T::Primitive {
        self.0
    }

    /// Transforms the float bits into the monotonically ordered Ordered space used by `total_cmp`.
    /// # Examples
    /// ```
    /// use range_set_blaze::FiniteF64;
    ///
    /// assert_eq!(2.0 < 3.0, FiniteF64::new(2.0).to_ordered() < FiniteF64::new(3.0).to_ordered());
    /// ```
    pub fn to_ordered(self) -> T::Ordered {
        T::to_ordered(self.0)
    }

    /// Transforms the ordered Ordered space back into standard float bits.
    /// # Examples
    /// ```
    /// use range_set_blaze::FiniteF64;
    ///
    /// assert_eq!(FiniteF64::from_ordered(FiniteF64::new(42.0).to_ordered()).into_inner(), 42.0);
    /// ```
    pub fn from_ordered(x: T::Ordered) -> Self {
        Self(T::from_ordered(x))
    }

    /// Returns the next float.
    ///
    /// # Examples
    /// ```
    /// use range_set_blaze::FiniteF64;
    ///
    /// assert_eq!(FiniteF64::new(42.0).next().prev().into_inner(), 42.0);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics on overflow if `self` is the maximum value.
    // TODO00: rust-version is now "1.87", so f32/f64::next_up() (stable since
    // 1.86) is available. Consider delegating to T::Primitive::next_up() here
    // instead of the manual to_ordered/wrapping_add/from_ordered round-trip
    // (still need the -0.0 skip check, just phrased on the primitive value).
    #[must_use]
    pub fn next(self) -> Self {
        debug_assert!(self != Self::MAX, "next() called on maximum value");
        let mut ordered = self.to_ordered();
        ordered = ordered.wrapping_add(&T::Ordered::one());
        if ordered == T::NEG_ZERO_ORDERED {
            ordered = ordered.wrapping_add(&T::Ordered::one());
        }
        Self::from_ordered(ordered)
    }

    /// Returns the previous float.
    ///
    /// # Examples
    /// ```
    /// use range_set_blaze::FiniteF64;
    ///
    /// assert_eq!(FiniteF64::new(42.0).prev().next().into_inner(), 42.0);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics on underflow if `self` is the minimum value.
    // TODO00: rust-version is now "1.87", so f32/f64::next_down() (stable
    // since 1.86) is available. Consider delegating to T::Primitive::
    // next_down() here instead of the manual to_ordered/wrapping_sub/
    // from_ordered round-trip (still need the -0.0 skip check, just phrased
    // on the primitive value).
    #[must_use]
    pub fn prev(self) -> Self {
        debug_assert!(self != Self::MIN, "prev() called on minimum value");
        let mut ordered = self.to_ordered();
        ordered = ordered.wrapping_sub(&T::Ordered::one());
        if ordered == T::NEG_ZERO_ORDERED {
            ordered = ordered.wrapping_sub(&T::Ordered::one());
        }
        Self::from_ordered(ordered)
    }

    /// Returns the next float.
    ///
    /// Returns [`None`] if `self` is the maximum value.
    ///
    /// # Examples
    /// ```
    /// use range_set_blaze::FiniteF64;
    ///
    /// let value = FiniteF64::new(42.0);
    /// assert_eq!(value.checked_next(), Some(value.next()));
    /// let value = FiniteF64::MAX;
    /// assert_eq!(value.checked_next(), None);
    /// ```
    #[must_use]
    pub fn checked_next(self) -> Option<Self> {
        if self == Self::MAX {
            None
        } else {
            Some(self.next())
        }
    }

    /// Returns the previous float.
    ///
    /// Returns [`None`] if `self` is the minimum value.
    ///
    /// # Examples
    /// ```
    /// use range_set_blaze::FiniteF64;
    ///
    /// let value = FiniteF64::new(42.0);
    /// assert_eq!(value.checked_prev(), Some(value.prev()));
    /// let value = FiniteF64::MIN;
    /// assert_eq!(value.checked_prev(), None);
    /// ```
    #[must_use]
    pub fn checked_prev(self) -> Option<Self> {
        if self == Self::MIN {
            None
        } else {
            Some(self.prev())
        }
    }

    /// Convenience method to convert an inclusive primitive range into an inclusive [`Finite`] range.
    /// # Examples
    /// ```
    /// use range_set_blaze::{RangeSetBlaze, FiniteF64};
    ///
    /// let short = RangeSetBlaze::from(FiniteF64::range(3.0..=5.0));
    /// let long = RangeSetBlaze::from(FiniteF64::new(3.0)..=FiniteF64::new(5.0));
    /// assert_eq!(short, long);
    #[must_use]
    pub fn range(range: RangeInclusive<T::Primitive>) -> RangeInclusive<Self> {
        let (start, end) = range.into_inner();
        Self(start)..=Self(end)
    }

    /// Convenience method to convert inclusive primitive ranges into inclusive [`Finite`] ranges.
    /// # Examples
    /// ```
    /// use range_set_blaze::{RangeSetBlaze, FiniteF64};
    ///
    /// let short = RangeSetBlaze::from_iter(FiniteF64::ranges([1.0..=2.0, 3.0..=4.0]));
    /// let long = RangeSetBlaze::from_iter([FiniteF64::new(1.0)..=FiniteF64::new(2.0), FiniteF64::new(3.0)..=FiniteF64::new(4.0)]);
    /// assert_eq!(short, long);
    pub fn ranges<I>(ranges: I) -> impl Iterator<Item = RangeInclusive<Self>>
    where
        I: IntoIterator<Item = RangeInclusive<T::Primitive>>,
    {
        ranges.into_iter().map(Self::range)
    }

    /// Convenience method to convert primitive values into ordered [`Finite`] values.
    /// # Examples
    /// ```
    /// use range_set_blaze::{RangeSetBlaze, FiniteF64};
    ///
    /// let short = RangeSetBlaze::from_iter(FiniteF64::values([1.0, 2.0, 3.0, 4.0]));
    /// let long = RangeSetBlaze::from_iter([FiniteF64::new(1.0), FiniteF64::new(2.0), FiniteF64::new(3.0), FiniteF64::new(4.0)]);
    /// assert_eq!(short, long);
    pub fn values<I>(values: I) -> impl Iterator<Item = Self>
    where
        I: IntoIterator<Item = T::Primitive>,
    {
        values.into_iter().map(Self)
    }

    /// Views primitive values as ordered [`Finite`] values.
    ///
    /// This runs in `O(1)` and does not allocate.
    /// # Examples
    /// ```
    /// use range_set_blaze::{RangeSetBlaze, FiniteF64};
    ///
    /// let short = RangeSetBlaze::from_iter(FiniteF64::slice(&[1.0, 2.0, 3.0, 4.0]));
    /// let long = RangeSetBlaze::from_iter([FiniteF64::new(1.0), FiniteF64::new(2.0), FiniteF64::new(3.0), FiniteF64::new(4.0)]);
    /// assert_eq!(short, long);
    #[must_use]
    pub const fn slice(values: &[T::Primitive]) -> &[Self] {
        // SAFETY: Finite is #[repr(transparent)] over T::Primitive, making `&[T::Primitive]`
        // and `&[Finite]` entirely interchangeable in layout and lifetimes.
        unsafe { core::mem::transmute::<&[T::Primitive], &[Self]>(values) }
    }
}

/// View [`Finite`] values as primitive values.
///
/// This runs in `O(1)` and does not allocate.
/// # Examples
/// ```
/// use range_set_blaze::FiniteF64;
/// use range_set_blaze::finite;
///
/// assert_eq!(&[1.0, 2.0, 3.0], finite::primitive_slice(&[FiniteF64::new(1.0), FiniteF64::new(2.0), FiniteF64::new(3.0)]))
#[must_use]
pub const fn primitive_slice<T: FiniteFloat>(values: &[Finite<T>]) -> &[T::Primitive] {
    // SAFETY: FiniteFloat is #[repr(transparent)] over T::Primitive, making `&[T::Primitive]`
    // and `&[FiniteFloat]` entirely interchangeable in layout and lifetimes.
    unsafe { core::mem::transmute::<&[Finite<T>], &[T::Primitive]>(values) }
}

impl<T: FiniteFloat> PartialEq for Finite<T> {
    fn eq(&self, other: &Self) -> bool {
        T::to_bits(self.0) == T::to_bits(other.0)
    }
}

impl<T: FiniteFloat> Eq for Finite<T> {}

impl<T: FiniteFloat> PartialOrd for Finite<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: FiniteFloat> Ord for Finite<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        T::total_cmp(self.0, other.0)
    }
}

impl<T: FiniteFloat> Hash for Finite<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        T::to_bits(self.0).hash(state);
    }
}

impl<T: FiniteFloat> crate::Integer for Finite<T> {
    type SafeLen = T::SafeLen;

    #[inline]
    fn checked_add_one(self) -> Option<Self> {
        self.checked_next()
    }

    // This moves to the next representable float in total_cmp order, not a numeric + 1.0.
    #[inline]
    fn add_one(self) -> Self {
        self.next()
    }

    #[inline]
    // This moves to the previous representable float in total_cmp order, not a numeric - 1.0.
    fn sub_one(self) -> Self {
        self.prev()
    }

    #[inline]
    fn assign_sub_one(&mut self) {
        *self = self.prev();
    }

    // Ideally, we would `impl std::iter::Step for FiniteF64` and just call Range::next(), but that's still experimental.
    #[inline]
    fn range_next(range: &mut RangeInclusive<Self>) -> Option<Self> {
        if range.is_empty() {
            None
        } else if range.start() == range.end() && *range.start() == Self::MAX {
            // This is cheating, but I think it still fulfills the contract
            let next = *range.start();
            *range = next..=range.end().prev();
            Some(next)
        } else {
            let next = *range.start();
            *range = (next.next())..=*range.end();
            Some(next)
        }
    }

    #[inline]
    fn range_next_back(range: &mut RangeInclusive<Self>) -> Option<Self> {
        if range.is_empty() {
            None
        } else if range.start() == range.end() && *range.start() == Self::MIN {
            // This is cheating, but I think it still fulfills the contract
            let last = *range.end();
            *range = last.next()..=last;
            Some(last)
        } else {
            let last = *range.end();
            *range = *range.start()..=last.prev();
            Some(last)
        }
    }

    #[inline]
    fn min_value() -> Self {
        Self::MIN
    }

    #[inline]
    fn max_value() -> Self {
        Self::MAX
    }

    #[cfg(feature = "from_slice")]
    #[inline]
    fn from_slice(slice: impl AsRef<[Self]>) -> RangeSetBlaze<Self> {
        // no way to do the fancy thing
        RangeSetBlaze::from_iter(slice.as_ref())
    }

    fn safe_len(r: &RangeInclusive<Self>) -> Self::SafeLen {
        T::prim_safe_len(r.start().into_inner(), r.end().into_inner())
    }

    fn safe_len_to_f64_lossy(len: Self::SafeLen) -> f64 {
        T::safe_len_to_f64_lossy(len)
    }

    fn f64_to_safe_len_lossy(f: f64) -> Self::SafeLen {
        T::f64_to_safe_len_lossy(f)
    }

    fn inclusive_end_from_start(self, b: Self::SafeLen) -> Self {
        self.inclusive_end_from_start(b)
    }

    fn start_from_inclusive_end(self, b: Self::SafeLen) -> Self {
        self.start_from_inclusive_end(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec;
    use std::vec::Vec;

    #[test]
    fn ordering_agrees_with_total_cmp() {
        let values = [-f64::MAX, -1.0, 0.0, 1.0, f64::MAX];

        for left in values {
            for right in values {
                assert_eq!(ff64(left).cmp(&ff64(right)), left.total_cmp(&right));
            }
        }
        assert_ne!(ff64(0.0).cmp(&ff64(-0.0)), 0.0_f64.total_cmp(&-0.0));
    }

    #[test]
    fn converts_ranges() {
        assert_eq!(FiniteF64::range(10.0..=20.0), ff64(10.0)..=ff64(20.0));
        assert_eq!(
            FiniteF64::ranges([10.0..=20.0, 30.0..=40.0]).collect::<Vec<_>>(),
            vec![ff64(10.0)..=ff64(20.0), ff64(30.0)..=ff64(40.0)]
        );
    }

    #[test]
    fn next_and_prev_step_through_zero_in_total_order() {
        assert_eq!(ff64(-0.0), ff64(0.0));
        assert_ne!(ff64(0.0).prev(), ff64(-0.0));
        assert_eq!(ff64(0.0).next(), ff64(f64::from_bits(1)));
        assert_eq!(
            ff64(0.0).prev(),
            ff64(f64::from_bits(0x8000_0000_0000_0001))
        );
    }

    #[test]
    fn next_and_prev_wrap() {
        // These should be true in release mode, but panic in debug as expected
        // assert_eq!(FiniteF64::MAX.next(), FiniteF64::MIN);
        // assert_eq!(FiniteF64::MIN.prev(), FiniteF64::MAX);
        assert_eq!(FiniteF64::MAX.checked_next(), None);
        assert_eq!(FiniteF64::MIN.checked_prev(), None);
    }

    #[test]
    fn checked_next_and_prev_stop_at_total_order_boundaries() {
        assert_eq!(FiniteF64::MIN.checked_prev(), None);
        assert_eq!(FiniteF64::MAX.checked_next(), None);
        assert_eq!(FiniteF64::MIN.checked_next(), Some(FiniteF64::MIN.next()));
        assert_eq!(FiniteF64::MAX.checked_prev(), Some(FiniteF64::MAX.prev()));
    }

    #[test]
    fn min_and_max_are_total_order_boundaries() {
        let values = [
            ff64(-f64::MAX),
            ff64(-1.0),
            ff64(-0.0),
            ff64(0.0),
            ff64(1.0),
            ff64(f64::MAX),
        ];

        for value in values {
            assert!(FiniteF64::MIN <= value);
            assert!(value <= FiniteF64::MAX);
        }
    }

    #[test]
    fn next_and_prev_are_neighbors_in_total_order() {
        let values = [
            ff64(f64::MIN),
            ff64(-f64::MAX),
            ff64(-1.0),
            ff64(-0.0),
            ff64(0.0),
            ff64(1.0),
            ff64(f64::MAX),
        ];

        for value in values {
            if value != ff64(f64::MAX) {
                assert_eq!(value.next().prev(), value);
            }
            if value != ff64(f64::MIN) {
                assert_eq!(value.prev().next(), value);
            }
        }
    }
}
