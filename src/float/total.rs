//! Total is a floating point type, suitable for use in ranges. All values are valid.
//! 
//! Ordering and other semantics are as per `total_cmp`.\
//! Every distinct bit pattern is a separate valid value, even though quite a few of them are NaN.\
//! For example, in a `Total<f32>` all 16 million different NAN values are distinct from each other.

use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
use core::ops::RangeInclusive;

use core::fmt::Debug;
use num_traits::One;
use num_traits::ops::wrapping::{WrappingAdd, WrappingSub};

#[cfg(feature = "from_slice")]
use crate::RangeSetBlaze;

use super::total_float::TotalFloat;

/// Total ordered f64, all values valid, including NaN, -0.0, +0.0, and infinities.
pub type TotalF64 = Total<f64>;
/// Total ordered f32, all values valid, including NaN, -0.0, +0.0, and infinities.
pub type TotalF32 = Total<f32>;
/// Total ordered f16, all values valid, including NaN, -0.0, +0.0, and infinities.
#[cfg(feature = "total_float_nightly_experimental")]
pub type TotalF16 = Total<f16>;
/// Total ordered f128, all values valid, including NaN, -0.0, +0.0, and infinities.
#[cfg(feature = "total_float_nightly_experimental")]
pub type TotalF128 = Total<f128>;

/// Construct a [`TotalF64`] from an `f64`. Shorthand for [`TotalF64::new`]
#[must_use]
pub const fn tf64(x: f64) -> TotalF64 {
    Total::<f64>::new(x)
}

/// Construct a [`TotalF32`] from an `f32`. Shorthand for [`TotalF32::new`]
#[must_use]
pub const fn tf32(x: f32) -> TotalF32 {
    Total::<f32>::new(x)
}

/// Construct a [`TotalF16`] from an `f16`. Shorthand for [`Total::<f16>::new`]
#[cfg(feature = "total_float_nightly_experimental")]
#[must_use]
pub const fn tf16(x: f16) -> TotalF16 {
    Total::<f16>::new(x)
}

/// Construct a [`TotalF128`] from an `f128`. Shorthand for [`Total::<f128>::new`]
#[cfg(feature = "total_float_nightly_experimental")]
#[must_use]
pub const fn tf128(x: f128) -> TotalF128 {
    Total::<f128>::new(x)
}

/// Experimental: A transparent wrapper around floating point values with total ordering.
///
/// Comparison, equality, and hashing all agree with `total_cmp`.
///
/// # Enabling
///
/// This type is experimental and must be enabled with the `total_float_experimental` feature.
/// ```bash
/// cargo add range-set-blaze --features "total_float_experimental"
/// ```
/// That provides the `Total32` and `Total64` types.
/// 
/// If you're building with nightly, you can instead use the `total_float_nightly_experimental` feature.
/// ```bash
/// cargo add range-set-blaze --features "total_float_nightly_experimental"
/// ```
/// To also use the `Total16` and `Total128` types.
#[repr(transparent)]
#[derive(Copy, Clone, Default, Debug)]
pub struct Total<T: TotalFloat>(T::Primitive);

impl<T: TotalFloat> Total<T> {
    /// The minimum value that can be represented by the type.
    /// I.e., the smallest possible value according to `total_cmp`\
    /// Maps directly to [`crate::Integer::min_value()`]
    ///
    /// # Examples
    /// ```
    /// use range_set_blaze::TotalF64;
    ///
    /// assert_eq!(TotalF64::MIN, TotalF64::new(f64::from_bits(u64::MAX)));
    /// ```
    pub const MIN: Self = Self(T::MIN);

    /// The maximum value that can be represented by the type.
    /// I.e., the smallest possible value according to `total_cmp`\
    /// Maps directly to [`crate::Integer::max_value()`]
    ///
    /// # Examples
    /// ```
    /// use range_set_blaze::TotalF64;
    ///
    /// assert_eq!(TotalF64::MAX, TotalF64::new(f64::from_bits(0x7fff_ffff_ffff_ffff)));
    /// ```
    pub const MAX: Self = Self(T::MAX);

    /// The maximum possible size of a range, i.e. the size if `[MIN..=MAX]`
    ///
    /// # Examples
    /// ```
    /// use range_set_blaze::TotalF32;
    ///
    /// assert_eq!(TotalF32::MAX_SIZE, u32::MAX as i64 + 1);
    /// ```
    pub const MAX_SIZE: T::SafeLen = T::MAX_SIZE;

    /// Creates a new [`Total`] from a primitive float.
    /// All values are legal. 
    ///
    /// # Examples
    /// ```
    /// use range_set_blaze::TotalF64;
    ///
    /// let _ = TotalF64::new(f64::INFINITY);
    /// ```
    #[must_use]
    pub const fn new(x: T::Primitive) -> Self {
        Self(x)
    }

    /// Computes `self + (b - 1)` where `b` is of type `SafeLen`.
    #[must_use]
    pub fn inclusive_end_from_start(self, b: T::SafeLen) -> Self {
        Self(T::inclusive_end_from_start(self.0, b))
    }

    /// Computes `self - (b - 1)` where `b` is of type `SafeLen`.
    #[must_use]
    pub fn start_from_inclusive_end(self, b: T::SafeLen) -> Self {
        Self(T::start_from_inclusive_end(self.0, b))
    }

    /// Returns the wrapped value.
    /// 
    /// # Examples
    /// ```
    /// use range_set_blaze::TotalF64;
    ///
    /// assert_eq!(TotalF64::new(42.0).into_inner(), 42.0);
    /// ```
    #[must_use]
    pub const fn into_inner(self) -> T::Primitive {
        self.0
    }

    /// Transforms the float bits into the monotonically ordered Ordered space used by `total_cmp`.
    /// # Examples
    /// ```
    /// use range_set_blaze::TotalF64;
    ///
    /// assert_eq!(2.0 < 3.0, TotalF64::new(2.0).to_ordered() < TotalF64::new(3.0).to_ordered());
    /// ```
    pub fn to_ordered(self) -> T::Ordered {
        T::to_ordered(self.0)
    }

    /// Transforms the ordered Ordered space back into standard float bits.
    /// # Examples
    /// ```
    /// use range_set_blaze::TotalF64;
    ///
    /// assert_eq!(TotalF64::from_ordered(TotalF64::new(42.0).to_ordered()).into_inner(), 42.0);
    /// ```
    pub fn from_ordered(x: T::Ordered) -> Self {
        Self(T::from_ordered(x))
    }

    /// Returns the next float in total order.
    ///
    /// Panics on overflow if `self` is the maximum value in total order.
    #[must_use]
    pub fn next(self) -> Self {
        debug_assert!(self != Self::MAX, "next() called on maximum value");
        let ordered = self.to_ordered();
        Self::from_ordered(ordered.wrapping_add(&T::Ordered::one()))
    }

    /// Returns the previous float in total order.
    ///
    /// Panics on overflow if `self` is the minimum value in total order.
    #[must_use]
    pub fn prev(self) -> Self {
        debug_assert!(self != Self::MIN, "prev() called on minimum value");
        let ordered = self.to_ordered();
        Self::from_ordered(ordered.wrapping_sub(&T::Ordered::one()))
    }

    /// Returns the next float in total order.
    ///
    /// Returns [`None`] if `self` is the maximum value in total order.
    #[must_use]
    pub fn checked_next(self) -> Option<Self> {
        if self == Self::MAX {
            None
        } else {
            Some(self.next())
        }
    }

    /// Returns the previous float in total order.
    ///
    /// Returns [`None`] if `self` is the minimum value in total order.
    #[must_use]
    pub fn checked_prev(self) -> Option<Self> {
        if self == Self::MIN {
            None
        } else {
            Some(self.prev())
        }
    }

    /// Converts an inclusive primitive range into an inclusive [`Total`] range.
    #[must_use]
    pub fn range(range: RangeInclusive<T::Primitive>) -> RangeInclusive<Self> {
        let (start, end) = range.into_inner();
        Self(start)..=Self(end)
    }

    /// Converts inclusive primitive ranges into inclusive [`Total`] ranges.
    pub fn ranges<I>(ranges: I) -> impl Iterator<Item = RangeInclusive<Self>>
    where
        I: IntoIterator<Item = RangeInclusive<T::Primitive>>,
    {
        ranges.into_iter().map(Self::range)
    }

    /// Converts primitive values into ordered [`Total`] values.
    pub fn values<I>(values: I) -> impl Iterator<Item = Self>
    where
        I: IntoIterator<Item = T::Primitive>,
    {
        values.into_iter().map(Self)
    }

    /// Views primitive values as ordered [`Total`] values.
    ///
    /// This runs in `O(1)` and does not allocate.
    #[must_use]
    pub const fn slice(values: &[T::Primitive]) -> &[Self] {
        // SAFETY: Total is #[repr(transparent)] over T::Primitive, making `&[T::Primitive]`
        // and `&[Total]` entirely interchangeable in layout and lifetimes.
        unsafe { core::mem::transmute::<&[T::Primitive], &[Self]>(values) }
    }
}

/// Views  [`Total`] values as primitive values.
///
/// This runs in `O(1)` and does not allocate.
#[must_use]
pub const fn primitive_slice<T: TotalFloat>(values: &[T]) -> &[T::Primitive] {
    // SAFETY: TotalFloat is #[repr(transparent)] over T::Primitive, making `&[T::Primitive]`
    // and `&[TotalFloat]` entirely interchangeable in layout and lifetimes.
    unsafe { core::mem::transmute::<&[T], &[T::Primitive]>(values) }
}

impl<T: TotalFloat> PartialEq for Total<T> {
    fn eq(&self, other: &Self) -> bool {
        T::to_bits(self.0) == T::to_bits(other.0)
    }
}

impl<T: TotalFloat> Eq for Total<T> {}

impl<T: TotalFloat> PartialOrd for Total<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: TotalFloat> Ord for Total<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        T::total_cmp(self.0, other.0)
    }
}

impl<T: TotalFloat> Hash for Total<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        T::to_bits(self.0).hash(state);
    }
}

impl<T: TotalFloat> crate::Integer for Total<T> {
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

    // Ideally, we would `impl std::iter::Step for TotalF64` and just call Range::next(), but that's still experimental.
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
    use std::collections::hash_map::DefaultHasher;
    use std::vec;
    use std::vec::Vec;

    #[test]
    fn ordering_agrees_with_total_cmp() {
        let values = [
            f64::NEG_INFINITY,
            -f64::MAX,
            -1.0,
            -0.0,
            0.0,
            1.0,
            f64::MAX,
            f64::INFINITY,
            f64::NAN,
            f64::from_bits(0x7ff8_0000_0000_0001),
            f64::from_bits(0xfff8_0000_0000_0001),
        ];

        for left in values {
            for right in values {
                assert_eq!(tf64(left).cmp(&tf64(right)), left.total_cmp(&right));
            }
        }
    }

    #[test]
    fn equality_agrees_with_total_cmp() {
        assert_ne!(tf64(-0.0), tf64(0.0));
        assert_eq!(tf64(f64::NAN), tf64(f64::NAN));
    }

    #[test]
    fn equal_values_hash_equally() {
        let left = hash(tf64(f64::NAN));
        let right = hash(tf64(f64::NAN));

        assert_eq!(left, right);
    }

    #[test]
    fn converts_ranges() {
        assert_eq!(TotalF64::range(10.0..=20.0), tf64(10.0)..=tf64(20.0));
        assert_eq!(
            TotalF64::ranges([10.0..=20.0, 30.0..=40.0]).collect::<Vec<_>>(),
            vec![tf64(10.0)..=tf64(20.0), tf64(30.0)..=tf64(40.0)]
        );
    }

    #[test]
    fn next_and_prev_step_through_zero_in_total_order() {
        assert_eq!(tf64(-0.0).next(), tf64(0.0));
        assert_eq!(tf64(0.0).prev(), tf64(-0.0));
        assert_eq!(tf64(0.0).next(), tf64(f64::from_bits(1)));
        assert_eq!(
            tf64(-0.0).prev(),
            tf64(f64::from_bits(0x8000_0000_0000_0001))
        );
    }

    #[test]
    fn next_and_prev_wrap() {
        // These should be true in release mode, but panic in debug as expected
        // assert_eq!(TotalF64::MAX.next(), TotalF64::MIN);
        // assert_eq!(TotalF64::MIN.prev(), TotalF64::MAX);
        assert_eq!(TotalF64::MAX.checked_next(), None);
        assert_eq!(TotalF64::MIN.checked_prev(), None);
    }

    #[test]
    fn next_and_prev_step_around_infinities() {
        assert_eq!(tf64(f64::MAX).next(), tf64(f64::INFINITY));
        assert_eq!(tf64(f64::INFINITY).prev(), tf64(f64::MAX));
        assert_eq!(tf64(f64::NEG_INFINITY).next(), tf64(-f64::MAX));
        assert_eq!(tf64(-f64::MAX).prev(), tf64(f64::NEG_INFINITY));
    }

    #[test]
    fn checked_next_and_prev_stop_at_total_order_boundaries() {
        assert_eq!(TotalF64::MIN.checked_prev(), None);
        assert_eq!(TotalF64::MAX.checked_next(), None);
        assert_eq!(TotalF64::MIN.checked_next(), Some(TotalF64::MIN.next()));
        assert_eq!(TotalF64::MAX.checked_prev(), Some(TotalF64::MAX.prev()));
    }

    #[test]
    fn min_and_max_are_total_order_boundaries() {
        let values = [
            tf64(f64::NEG_INFINITY),
            tf64(-f64::MAX),
            tf64(-1.0),
            tf64(-0.0),
            tf64(0.0),
            tf64(1.0),
            tf64(f64::MAX),
            tf64(f64::INFINITY),
            tf64(f64::NAN),
            tf64(f64::from_bits(0x7ff8_0000_0000_0001)),
            tf64(f64::from_bits(0xfff8_0000_0000_0001)),
        ];

        for value in values {
            assert!(TotalF64::MIN <= value);
            assert!(value <= TotalF64::MAX);
        }
    }

    #[test]
    fn next_and_prev_are_neighbors_in_total_order() {
        let values = [
            tf64(f64::NEG_INFINITY),
            tf64(-f64::MAX),
            tf64(-1.0),
            tf64(-0.0),
            tf64(0.0),
            tf64(1.0),
            tf64(f64::MAX),
            tf64(f64::INFINITY),
            tf64(f64::NAN),
            tf64(f64::from_bits(0x7ff8_0000_0000_0001)),
            tf64(f64::from_bits(0xfff8_0000_0000_0001)),
        ];

        for value in values {
            assert_eq!(value.next().prev(), value);
            assert_eq!(value.prev().next(), value);
        }
    }

    fn hash(value: TotalF64) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }
    #[test]
    #[cfg(feature = "total_float_nightly_experimental")]
    fn ordered_round_trip() {
        use crate::total_float::from_ordered_16;
        use crate::total_float::to_ordered_16;
        for x in i16::MIN..=i16::MAX {
            assert_eq!(to_ordered_16(from_ordered_16(x)), x);
        }
    }
}
