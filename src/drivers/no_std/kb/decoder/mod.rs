mod debounce;
mod key_codes;
mod key_mapping;
mod keyscan;
mod matrix;

pub use debounce::*;
pub use keyscan::*;
pub use matrix::*;

pub const NUM_COLS: usize = 8;
pub const NUM_ROWS: usize = 10;
pub const NUM_MODS: usize = 5;
