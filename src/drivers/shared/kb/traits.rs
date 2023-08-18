use super::Key;
use core::{convert::Infallible, ops::Deref};
use cortex_m::delay::Delay;

use embedded_hal::digital::v2::{InputPin, OutputPin};

use crate::drivers::no_std::kb::decoder::{Debounce, NUM_COLS, NUM_MODS, NUM_ROWS};

#[cfg(feature = "no-std")]
use crate::drivers::no_std::kb::input::KbDriverInput;

#[cfg(feature = "no-std")]
use crate::drivers::no_std::kb::oracle::KbOracleReports;

#[cfg(feature = "no-std")]
use usbd_hid::descriptor::KeyboardReport;

#[cfg(feature = "no-std")]
use alloc::vec::Vec;

pub trait KeyboardDriver {
    #[cfg(feature = "no-std")]
    fn init() -> Self;
    #[cfg(feature = "no-std")]
    fn process_key_event(
        &mut self,
        modifiers: (
            &[&dyn InputPin<Error = Infallible>],
            &[&dyn InputPin<Error = Infallible>],
        ),
        rows: &[&dyn InputPin<Error = Infallible>],
        columns: &mut [&mut dyn embedded_hal::digital::v2::OutputPin<Error = Infallible>],
        delay: &mut Delay,
        debounce: &mut Debounce<NUM_MODS, NUM_ROWS, NUM_COLS>,
    ) -> Option<Vec<KbOracleReports>>;
    #[cfg(feature = "no-std")]
    fn hid_report(self) -> Vec<KeyboardReport>;
}

pub trait KeyboardKeyMap {
    #[cfg(feature = "no-std")]
    fn find_input(self, layer: u8, scan_code: u8) -> Option<(Key, KbDriverInput)>;
}
