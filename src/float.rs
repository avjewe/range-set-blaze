//! Experimental support for floating point ranges
//! Enable with `total_float_experimental` (stable, `f32`/`f64`) and
//! `total_float_nightly_experimental` (nightly, adds `f16`/`f128`).

pub mod finite_float;
pub mod total_float;

pub mod total;
pub use total::{Total, TotalF32, TotalF64};
#[cfg(feature = "total_float_nightly_experimental")]
pub use total::{TotalF16, TotalF128};

pub mod finite;
pub use finite::{Finite, FiniteF32, FiniteF64};
#[cfg(feature = "total_float_nightly_experimental")]
pub use finite::{FiniteF16, FiniteF128};
