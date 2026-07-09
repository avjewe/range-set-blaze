//! Experimental support for floating point ranges.\
//! Enable with `total_float_experimental` (stable, `f32`/`f64`) and
//! `total_float_nightly_experimental` (nightly, adds `f16`/`f128`).
//!
//! Exports two types of floating point range types.\
//! Total: Every bit pattern is valid and distinct.\
//! Finite: Only finite floating point values are valid, e.g. `f64::MIN..=f64::MAX`. Also, -0.0 is treated as 0.0
//!
//! Each of those is available in four sizes: 16, 32, 64 and 128.
//!
//!```
//! use range_set_blaze::{RangeSetBlaze, FiniteF64, TotalF32, TotalF64};
//! let set = RangeSetBlaze::from_iter([TotalF64::new(3.0)..=TotalF64::new(5.0)]);
//! assert!(set.contains(TotalF64::new(3.1)));
//! assert!(!set.contains(TotalF64::new(2.9)));
//!
//! let set = RangeSetBlaze::from(FiniteF64::range(3.0..=5.0));
//! assert!(set.contains(FiniteF64::new(4.9)));
//! assert!(!set.contains(FiniteF64::new(5.1)));
//!
//! let set = RangeSetBlaze::from_iter(TotalF32::ranges([3.0..=5.0, 7.0..=9.0]));
//! assert!(set.contains(TotalF32::new(4.0)));
//! assert!(!set.contains(TotalF32::new(6.0)));
//!```
//!
//! ## Full worked example
//!
//! `examples/float_maps.rs` builds a `RangeMapBlaze<FiniteF32, u8>` dispatch table by sweeping
//! every finite `f32` in `[-pi, pi]`, computing how many Taylor-series terms `cos(x)` needs at
//! each point. Its full source (kept in sync automatically via `include_str!`) is below.
#![doc = concat!(
    "````rust,no_run\n",
    include_str!("../examples/float_maps.rs"),
    "\n````"
)]

#[doc(hidden)]
pub mod finite_float;
#[doc(hidden)]
pub mod total_float;

pub mod total;
pub use total::{Total, TotalF32, TotalF64};
#[cfg(feature = "total_float_nightly_experimental")]
pub use total::{TotalF16, TotalF128};

pub mod finite;
pub use finite::{Finite, FiniteF32, FiniteF64};
#[cfg(feature = "total_float_nightly_experimental")]
pub use finite::{FiniteF16, FiniteF128};
