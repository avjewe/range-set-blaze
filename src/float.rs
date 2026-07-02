//! Experimental support for floating point ranges
//! Enable with `total_float_experimental` (stable, `f32`/`f64`) and
//! `total_float_nightly_experimental` (nightly, adds `f16`/`f128`).

#[allow(clippy::module_inception)]
pub mod float;

pub mod total;
#[cfg(feature = "total_float_nightly_experimental")]
pub use total::TotalF16;
pub use total::{Total, TotalF32, TotalF64};

pub mod finite;
#[cfg(feature = "total_float_nightly_experimental")]
pub use finite::FiniteF16;
pub use finite::{Finite, FiniteF32, FiniteF64};
