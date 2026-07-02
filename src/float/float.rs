//! Ordered `f64` support.

use core::cmp::Ordering;
use core::hash::Hash;

use num_traits::ops::checked::{CheckedAdd, CheckedSub};
use num_traits::ops::wrapping::{WrappingAdd, WrappingSub};
use num_traits::{Num, One, Zero};
use std::fmt::{Debug, Display};

/// Minimum scaffolding necessary to implement Total and Finite for a floating point type.
pub trait Float: Default + Copy + Clone + Debug + Send + Sync + 'static {
    /// The wrapped type, e.g. f64
    type Primitive: Float;
    /// The result of `to_bits()` on the wrapped type, e.g. u64
    type Bits: Num + Copy + Hash + Send + Sync + Debug;
    /// The intermediate type used for comparison, e.g. i64
    type Signed: CheckedAdd
        + CheckedSub
        + WrappingAdd
        + WrappingSub
        + One
        + PartialEq
        + Copy
        + Send
        + Sync
        + Debug
        + Display;
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
        + std::ops::AddAssign
        + std::ops::SubAssign;

    /// The minimum value available, in the `total_cmp`, range-set sense
    const MIN: Self::Primitive;
    /// The maximum value available, in the `total_cmp`, range-set sense
    const MAX: Self::Primitive;

    /// The minimum value available, in the usual floating point sense
    const MIN_FINITE: Self::Primitive;
    /// The maximum value available, in the usual floating point sense
    const MAX_FINITE: Self::Primitive;

    /// Is this the ordered value of negative zero?
    /// This should be `const NEG_ZERO: Self::Signed;` but const traits aren't ready yet
    fn is_neg_zero(x: Self::Signed) -> bool;

    /// Transform Primitive into Signed, to allow comparison and addition
    fn to_ordered(x: Self::Primitive) -> Self::Signed;
    /// Transform Signed back to Primitive, presumably after some addition
    fn from_ordered(x: Self::Signed) -> Self::Primitive;
    /// Transform Primitive into a type with more concrete semantics
    fn to_bits(x: Self::Primitive) -> Self::Bits;
    /// Return the size of the inclusive range from start to end
    fn safe_len(start: Self::Signed, end: Self::Signed) -> Self::SafeLen;
    /// Converts [`SafeLen`] to `f64`, potentially losing precision for large values.
    fn safe_len_to_f64_lossy(len: Self::SafeLen) -> f64;
    /// Converts a `f64` to [`SafeLen`] using the formula `f as Self::SafeLen`. For large integer types, this will result in a loss of precision.
    fn f64_to_safe_len_lossy(f: f64) -> Self::SafeLen;
    /// return (x - 1) as `Self::Signed`
    fn safe_as_signed(x: Self::SafeLen) -> Self::Signed;
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
        Self::from_ordered(Self::to_ordered(a).wrapping_add(&Self::safe_as_signed(b)))
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
        Self::from_ordered(Self::to_ordered(a).wrapping_sub(&Self::safe_as_signed(b)))
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

impl Float for f64 {
    type Primitive = Self;
    type Bits = u64;
    type Signed = i64;
    type SafeLen = i128;

    const MIN: Self = Self::from_bits(u64::MAX);
    const MAX: Self = Self::from_bits(0x7fff_ffff_ffff_ffff);
    const MIN_FINITE: Self = Self::MIN;
    const MAX_FINITE: Self = Self::MAX;

    fn to_bits(x: Self::Primitive) -> Self::Bits {
        x.to_bits()
    }

    /// Transforms the float bits into the monotonically ordered `i64` space used by `total_cmp`.
    fn to_ordered(x: Self::Primitive) -> Self::Signed {
        let mut bits = x.to_bits().cast_signed();
        bits ^= ((bits >> 63).cast_unsigned() >> 1).cast_signed();
        bits
    }

    /// Transforms the ordered `i64` space back into standard float bits.
    fn from_ordered(mut bits: Self::Signed) -> Self::Primitive {
        // Reversing the XOR transformation
        bits ^= ((bits >> 63).cast_unsigned() >> 1).cast_signed();
        Self::from_bits(bits.cast_unsigned())
    }

    fn safe_len(start: Self::Signed, end: Self::Signed) -> Self::SafeLen {
        // 1️⃣ Contract: caller promises start ≤ end  (checked only in debug builds)
        debug_assert!(start <= end, "start ≤ end required");

        // 2️⃣ Compute distance in `Self` then reinterpret‑cast to the first
        Self::SafeLen::from(end) - Self::SafeLen::from(start) + 1
    }

    #[expect(clippy::cast_precision_loss)]
    #[expect(clippy::use_self, reason = "f64 is not really Self")]
    fn safe_len_to_f64_lossy(len: Self::SafeLen) -> f64 {
        len as f64
    }

    #[expect(clippy::cast_possible_truncation)]
    fn f64_to_safe_len_lossy(f: f64) -> Self::SafeLen {
        f as Self::SafeLen
    }

    #[expect(clippy::cast_possible_truncation)]
    fn safe_as_signed(x: Self::SafeLen) -> Self::Signed {
        (x - 1) as Self::Signed
    }
    fn total_cmp(x: Self::Primitive, y: Self::Primitive) -> Ordering {
        x.total_cmp(&y)
    }
    fn is_neg_zero(x: Self::Signed) -> bool {
        x == Self::to_ordered(-0.0)
    }

    fn is_finite(x: Self::Primitive) -> bool {
        x.is_finite()
    }
    fn normalize(x: Self::Primitive) -> Self::Primitive {
        const NEG_ZERO: u64 = f64::to_bits(-0.0);
        if x.to_bits() == NEG_ZERO { 0.0 } else { x }
    }
}

impl Float for f32 {
    type Primitive = Self;
    type Bits = u32;
    type Signed = i32;
    type SafeLen = i64;

    const MIN: Self = Self::from_bits(u32::MAX);
    const MAX: Self = Self::from_bits(0x7fff_ffff);
    const MIN_FINITE: Self = Self::MIN;
    const MAX_FINITE: Self = Self::MAX;

    fn to_bits(x: Self::Primitive) -> Self::Bits {
        x.to_bits()
    }

    /// Transforms the float bits into the monotonically ordered `i64` space used by `total_cmp`.
    fn to_ordered(x: Self::Primitive) -> Self::Signed {
        let mut bits = x.to_bits().cast_signed();
        bits ^= ((bits >> 31).cast_unsigned() >> 1).cast_signed();
        bits
    }

    /// Transforms the ordered `i64` space back into standard float bits.
    fn from_ordered(mut bits: Self::Signed) -> Self::Primitive {
        // Reversing the XOR transformation
        bits ^= ((bits >> 31).cast_unsigned() >> 1).cast_signed();
        Self::from_bits(bits.cast_unsigned())
    }

    fn safe_len(start: Self::Signed, end: Self::Signed) -> Self::SafeLen {
        // 1️⃣ Contract: caller promises start ≤ end  (checked only in debug builds)
        debug_assert!(start <= end, "start ≤ end required");

        // 2️⃣ Compute distance in `Self` then reinterpret‑cast to the first
        Self::SafeLen::from(end) - Self::SafeLen::from(start) + 1
    }

    #[allow(clippy::cast_precision_loss)]
    fn safe_len_to_f64_lossy(len: Self::SafeLen) -> f64 {
        len as f64
    }

    #[expect(clippy::cast_possible_truncation)]
    fn f64_to_safe_len_lossy(f: f64) -> Self::SafeLen {
        f as Self::SafeLen
    }

    #[expect(clippy::cast_possible_truncation)]
    fn safe_as_signed(x: Self::SafeLen) -> Self::Signed {
        (x - 1) as Self::Signed
    }
    fn total_cmp(x: Self::Primitive, y: Self::Primitive) -> Ordering {
        x.total_cmp(&y)
    }
    fn is_neg_zero(x: Self::Signed) -> bool {
        x == Self::to_ordered(-0.0f32)
    }
    fn is_finite(x: Self::Primitive) -> bool {
        x.is_finite()
    }
    fn normalize(x: Self::Primitive) -> Self::Primitive {
        const NEG_ZERO: u32 = f32::to_bits(-0.0);
        if x.to_bits() == NEG_ZERO { 0.0 } else { x }
    }
}

#[cfg(feature = "total_float_nightly_experimental")]
impl Float for f16 {
    type Primitive = Self;
    type Bits = u16;
    type Signed = i16;
    type SafeLen = i32;

    const MIN: Self = Self::from_bits(u16::MAX);
    const MAX: Self = Self::from_bits(0x7fff);
    const MIN_FINITE: Self = Self::MIN;
    const MAX_FINITE: Self = Self::MAX;

    fn to_bits(x: Self::Primitive) -> Self::Bits {
        x.to_bits()
    }

    /// Transforms the float bits into the monotonically ordered `i64` space used by `total_cmp`.
    fn to_ordered(x: Self::Primitive) -> Self::Signed {
        let mut bits = x.to_bits().cast_signed();
        bits ^= ((bits >> 15).cast_unsigned() >> 1).cast_signed();
        bits
    }

    /// Transforms the ordered `i64` space back into standard float bits.
    fn from_ordered(mut bits: Self::Signed) -> Self::Primitive {
        // Reversing the XOR transformation
        bits ^= ((bits >> 15).cast_unsigned() >> 1).cast_signed();
        Self::from_bits(bits.cast_unsigned())
    }

    fn safe_len(start: Self::Signed, end: Self::Signed) -> Self::SafeLen {
        // 1️⃣ Contract: caller promises start ≤ end  (checked only in debug builds)
        debug_assert!(start <= end, "start ≤ end required");

        // 2️⃣ Compute distance in `Self` then reinterpret‑cast to the first
        Self::SafeLen::from(end) - Self::SafeLen::from(start) + 1
    }

    #[allow(clippy::cast_precision_loss)]
    fn safe_len_to_f64_lossy(len: Self::SafeLen) -> f64 {
        f64::from(len)
    }

    #[expect(clippy::cast_possible_truncation)]
    fn f64_to_safe_len_lossy(f: f64) -> Self::SafeLen {
        f as Self::SafeLen
    }

    #[expect(clippy::cast_possible_truncation)]
    fn safe_as_signed(x: Self::SafeLen) -> Self::Signed {
        (x - 1) as Self::Signed
    }
    fn total_cmp(x: Self::Primitive, y: Self::Primitive) -> Ordering {
        x.total_cmp(&y)
    }
    fn is_neg_zero(x: Self::Signed) -> bool {
        x == Self::to_ordered(-0.0f16)
    }
    fn is_finite(x: Self::Primitive) -> bool {
        x.is_finite()
    }
    fn normalize(x: Self::Primitive) -> Self::Primitive {
        const NEG_ZERO: u16 = f16::to_bits(-0.0);
        if x.to_bits() == NEG_ZERO { 0.0 } else { x }
    }
}
