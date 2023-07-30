#[cfg(feature = "std")]
mod std;

#[cfg(feature = "no-std")]
pub mod no_std;

#[cfg(feature = "std")]
pub use std as drivers;

pub mod shared;
