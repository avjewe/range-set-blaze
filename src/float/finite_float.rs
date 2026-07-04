//! Ordered `f64` support.

use core::cmp::Ordering;
use core::hash::Hash;

#[cfg(feature = "total_float_nightly_experimental")]
use super::total_float::{from_ordered_16, from_ordered_128, to_ordered_16, to_ordered_128};
use super::total_float::{from_ordered_32, from_ordered_64, to_ordered_32, to_ordered_64};
use core::fmt::{Debug, Display};
use num_traits::ops::wrapping::{WrappingAdd, WrappingSub};
use num_traits::{Num, One, Zero};

/// Minimum scaffolding necessary to implement Total and Finite for a floating point type.
pub trait FiniteFloat: Default + Copy + Clone + Debug + Send + Sync + 'static {
    /// The wrapped type, e.g. f64
    type Primitive: FiniteFloat;
    /// The result of `to_bits()` on the wrapped type, e.g. u64
    type Bits: Num + Copy + Hash + Send + Sync + Debug;
    /// The intermediate type used for comparison, e.g. i64
    type Ordered: WrappingAdd
        + WrappingSub
        + One
        + PartialEq
        + Copy
        + Send
        + Sync
        + Debug
        + Display
        + PartialOrd;
    /// Integral type for holding size of any range. Must hold at least one more value than `Bits`.
    type SafeLen: Send
        + Sync
        + Debug
        + Display
        // Needed for Integer::SafeLen
        + Hash
        + Copy
        + PartialEq
        + PartialOrd
        + num_traits::Zero
        + num_traits::One
        + core::ops::AddAssign
        + core::ops::SubAssign;

    /// The minimum value available, in the usual floating point sense
    const MIN: Self::Primitive;
    /// The maximum value available, in the usual floating point sense
    const MAX: Self::Primitive;

    /// The minimum value available, in the usual floating point sense
    const MIN_ORDERED: Self::Ordered;
    /// The maximum value available, in the usual floating point sense
    const MAX_ORDERED: Self::Ordered;

    /// The maximum possible size of a range, i.e. the maximum value possible from `safe_len()`
    const MAX_SIZE: Self::SafeLen;

    /// The bit pattern for negative zero
    const NEG_ZERO_BITS: Self::Bits;

    /// The ordered value of negative zero
    const NEG_ZERO_ORDERED: Self::Ordered;

    /// Transform Primitive into Ordered, to allow comparison and addition
    fn to_ordered(x: Self::Primitive) -> Self::Ordered;
    /// Transform Ordered back to Primitive, presumably after some addition
    fn from_ordered(x: Self::Ordered) -> Self::Primitive;
    /// Transform Primitive into a type with more concrete semantics
    fn to_bits(x: Self::Primitive) -> Self::Bits;
    /// Return the size of the inclusive range from start to end
    fn safe_len(start: Self::Ordered, end: Self::Ordered) -> Self::SafeLen;
    /// Converts [`SafeLen`] to `f64`, potentially losing precision for large values.
    fn safe_len_to_f64_lossy(len: Self::SafeLen) -> f64;
    /// Converts a `f64` to [`SafeLen`] using the formula `f as Self::SafeLen`. For large integer types, this will result in a loss of precision.
    fn f64_to_safe_len_lossy(f: f64) -> Self::SafeLen;
    /// return (x - 1) as `Self::Ordered`
    fn safe_as_ordered(x: Self::SafeLen) -> Self::Ordered;
    /// Returns the ordering between `x` and `y`, as per the standard library's `f64::total_cmp`.
    /// Needed because f16 is not supported in `num_traits`.
    fn total_cmp(x: Self::Primitive, y: Self::Primitive) -> Ordering;

    /// Computes `self + (b - 1)` where `b` is of type [`SafeLen`].
    fn inclusive_end_from_start(a: Self::Primitive, b: Self::SafeLen) -> Self::Primitive {
        #[cfg(debug_assertions)]
        {
            let max_len = Self::prim_safe_len(a, Self::MAX);
            assert!(
                Self::SafeLen::zero() < b && b <= max_len,
                "b must be in range 1..=max_len (b = {b}, max_len = {max_len})"
            );
        }
        // If b is in range, two’s-complement wrap-around yields the correct inclusive end even if the add overflows
        let start = Self::to_ordered(a);
        let mut end = start.wrapping_add(&Self::safe_as_ordered(b));
        if (start..=end).contains(&Self::NEG_ZERO_ORDERED) {
            end = end.wrapping_add(&Self::Ordered::one());
        }
        Self::from_ordered(end)
    }
    /// Computes `self - (b - 1)` where `b` is of type [`Integer::SafeLen`].
    fn start_from_inclusive_end(a: Self::Primitive, b: Self::SafeLen) -> Self::Primitive {
        #[cfg(debug_assertions)]
        {
            let max_len = Self::prim_safe_len(Self::MIN, a);
            assert!(
                Self::SafeLen::zero() < b && b <= max_len,
                "b must be in range 1..=max_len (b = {b}, max_len = {max_len})"
            );
        }
        // If b is in range, two’s-complement wrap-around yields the correct start even if the sub overflows
        let end = Self::to_ordered(a);
        let mut start = end.wrapping_sub(&Self::safe_as_ordered(b));
        if (start..=end).contains(&Self::NEG_ZERO_ORDERED) {
            start = start.wrapping_sub(&Self::Ordered::one());
        }
        Self::from_ordered(start)
    }
    /// Return the size of the inclusive range from start to end.
    fn prim_safe_len(start: Self::Primitive, end: Self::Primitive) -> Self::SafeLen {
        Self::safe_len(Self::to_ordered(start), Self::to_ordered(end))
    }
    /// Return true if the float is finite.
    fn is_finite(x: Self::Primitive) -> bool;
    /// Turn negative zero into positive zero, leave other numbers unchanged.
    fn normalize(x: Self::Primitive) -> Self::Primitive;
}

macro_rules! impl_finite_ops {
    ($to_ordered:ident) => {
        const MIN: Self = Self::MIN;
        const MAX: Self = Self::MAX;
        const MIN_ORDERED: Self::Ordered = $to_ordered(Self::MIN);
        const MAX_ORDERED: Self::Ordered = $to_ordered(Self::MAX);
        const NEG_ZERO_BITS: Self::Bits = Self::to_bits(-0.0);
        const NEG_ZERO_ORDERED: Self::Ordered = $to_ordered(-0.0);

        fn to_ordered(x: Self::Primitive) -> Self::Ordered {
            $to_ordered(x)
        }

        fn to_bits(x: Self::Primitive) -> Self::Bits {
            x.to_bits()
        }
        #[expect(clippy::cast_sign_loss)]
        fn safe_len(start: Self::Ordered, end: Self::Ordered) -> Self::SafeLen {
            // 1️⃣ Contract: caller promises start ≤ end  (checked only in debug builds)
            debug_assert!(start <= end, "start ≤ end required");
            debug_assert!(start >= Self::MIN_ORDERED, "start >= MIN required");
            debug_assert!(end <= Self::MAX_ORDERED, "end <= MAX required");

            if (start..=end).contains(&Self::NEG_ZERO_ORDERED) {
                end.wrapping_sub(start) as Self::SafeLen
            } else {
                end.wrapping_sub(start).wrapping_add(1) as Self::SafeLen
            }
        }

        #[allow(clippy::cast_precision_loss)]
        #[allow(clippy::use_self, reason = "f64 is not really Self")]
        #[allow(clippy::cast_lossless)]
        fn safe_len_to_f64_lossy(len: Self::SafeLen) -> f64 {
            len as f64
        }

        #[expect(clippy::cast_possible_truncation)]
        #[expect(clippy::cast_sign_loss)]
        fn f64_to_safe_len_lossy(f: f64) -> Self::SafeLen {
            f as Self::SafeLen
        }

        #[expect(clippy::cast_possible_wrap)]
        fn safe_as_ordered(x: Self::SafeLen) -> Self::Ordered {
            (x - 1) as Self::Ordered
        }

        fn total_cmp(x: Self::Primitive, y: Self::Primitive) -> Ordering {
            x.total_cmp(&y)
        }

        fn is_finite(x: Self::Primitive) -> bool {
            x.is_finite()
        }

        fn normalize(x: Self::Primitive) -> Self::Primitive {
            if x.to_bits() == Self::NEG_ZERO_BITS {
                0.0
            } else {
                x
            }
        }
    };
}

impl FiniteFloat for f64 {
    type Primitive = Self;
    type Bits = u64;
    type Ordered = i64;
    type SafeLen = u64;

    const MAX_SIZE: Self::SafeLen = 0xFFE0_0000_0000_0000_u64 - 1;

    fn from_ordered(bits: Self::Ordered) -> Self::Primitive {
        from_ordered_64(bits)
    }

    impl_finite_ops!(to_ordered_64);
}

impl FiniteFloat for f32 {
    type Primitive = Self;
    type Bits = u32;
    type Ordered = i32;
    type SafeLen = u32;

    const MAX_SIZE: Self::SafeLen = 0xFF00_0000_u32 - 1;

    impl_finite_ops!(to_ordered_32);

    fn from_ordered(bits: Self::Ordered) -> Self::Primitive {
        from_ordered_32(bits)
    }
}

#[cfg(feature = "total_float_nightly_experimental")]
impl FiniteFloat for f16 {
    type Primitive = Self;
    type Bits = u16;
    type Ordered = i16;
    type SafeLen = u16;

    const MAX_SIZE: Self::SafeLen = 0xF800u16 - 1;

    impl_finite_ops!(to_ordered_16);

    fn from_ordered(bits: Self::Ordered) -> Self::Primitive {
        from_ordered_16(bits)
    }
}

#[cfg(feature = "total_float_nightly_experimental")]
impl FiniteFloat for f128 {
    type Primitive = Self;
    type Bits = u128;
    type Ordered = i128;
    type SafeLen = u128;

    const MAX_SIZE: Self::SafeLen = 0xFFFE_0000_0000_0000_0000_0000_0000_0000_u128 - 1;

    impl_finite_ops!(to_ordered_128);

    fn from_ordered(bits: Self::Ordered) -> Self::Primitive {
        from_ordered_128(bits)
    }
}
