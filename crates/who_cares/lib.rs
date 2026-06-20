#![feature(try_trait_v2)]

pub mod who_cares;

pub use who_cares::WhoCares;
#[cfg(feature = "macro")]
pub use who_cares_macros::who_cares;
