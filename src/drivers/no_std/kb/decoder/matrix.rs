use alloc::vec::Vec;
use rp2040_hal::gpio::{Input, Pins};

pub struct DecoderPin {}

pub struct DecoderMatix {
    pub pins: Vec<Vec<u64>>,
}

impl DecoderMatix {
    pub fn init(pins: Pins) -> Self {
        //let in_pin = pins.gpio26.into_mode::<Input<>>();

        Self {
            pins: [[].to_vec()].to_vec(),
        }
    }
    pub fn read_pins() -> Self {
        Self { pins: [].to_vec() }
    }
}
