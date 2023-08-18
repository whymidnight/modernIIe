mod hid;

use crate::drivers::shared::kb::{Key, KeyboardKeyMap};
use crate::utils::hex::{self, u8_to_hex_string};

use alloc::vec::*;

use self::hid::hoist_hid_keyboard_map;

#[cfg(feature = "no-std")]
use super::input::KbDriverInput;
use super::input::KEY_ASCII;
#[cfg(feature = "no-std")]
pub use hid::KeyboardMapEntrant;

pub type LayoutKeyWithHIDEntrant = (u8, u8, Vec<KeyboardMapEntrant>);
pub type LayoutKeyWithHID = (&'static str, LayoutKeyWithHIDEntrant);
pub type LayoutKey = (&'static str, (u8, u8));
pub type Layout = [(&'static str, [LayoutKey; 128]); 1];
pub type LayoutWithHID = Vec<(&'static str, Vec<LayoutKeyWithHID>)>;

#[derive(Clone)]
pub struct KeyMap {
    pub layout: Vec<Option<Vec<Option<LayoutKeyWithHIDEntrant>>>>,
}

#[cfg(feature = "no-std")]
impl KeyMap {
    pub fn init() -> KeyMap {
        let hid = hoist_hid_keyboard_map();

        let mut layers: Vec<Option<Vec<Option<LayoutKeyWithHIDEntrant>>>> = Vec::new();
        for _ in 0..0xC5 {
            layers.push(None);
        }

        {
            hid.iter().for_each(|layer| {
                let mut layout: Vec<Option<LayoutKeyWithHIDEntrant>> = Vec::new();
                // backfill layout
                for _ in 0..256 {
                    layout.push(None);
                }

                let layer_mask = layer.0;
                layer.1.iter().for_each(|layer_key| {
                    let key_up_code: u8 = {
                        let asdf = hex::decode_hex(layer_key.0.strip_prefix("0x").unwrap());
                        asdf
                    };
                    layout[key_up_code as usize] =
                        Some((layer_key.1 .0, layer_key.1 .1, layer_key.1 .2.clone()));
                });

                let layer_mask_parsed: u8 = {
                    let asdf = hex::decode_hex(layer_mask.strip_prefix("0x").unwrap());
                    asdf
                };
                layers[layer_mask_parsed as usize] = Some(layout);
            });
        }
        Self { layout: layers }
    }
}

impl KeyboardKeyMap for KeyMap {
    fn find_input(self, layer: u8, scan_code: u8) -> Option<(Key, KbDriverInput)> {
        match &self.layout[layer as usize] {
            Some(layout) => {
                let scan_code_input = scan_code & KEY_ASCII;
                let input_found = &layout[scan_code_input as usize];
                match input_found {
                    Some(input) => {
                        let (key_up, key_down, usb_hid) = input;

                        let (input_key, driver_input) = if scan_code == scan_code_input {
                            (
                                Key::define(key_down.clone(), key_up.clone(), usb_hid.to_vec()),
                                KbDriverInput::KeyUp,
                            )
                        } else {
                            (
                                Key::define(key_down.clone(), key_up.clone(), usb_hid.to_vec()),
                                KbDriverInput::KeyDown,
                            )
                        };

                        match driver_input {
                            KbDriverInput::KeyUp => Some((
                                input_key.clone(),
                                KbDriverInput::from_key(layer, input_key, driver_input),
                            )),
                            KbDriverInput::KeyDown => Some((
                                input_key.clone(),
                                KbDriverInput::from_key(layer, input_key, driver_input),
                            )),
                            _ => None,
                        }
                    }
                    None => {
                        defmt::error!(
                            "UNABLE TO FIND layout for {} input for {} :::::",
                            u8_to_hex_string(layer).as_str(),
                            u8_to_hex_string(scan_code).as_str(),
                        );
                        None
                    }
                }
            }
            None => {
                let a = u8_to_hex_string(layer);
                let b = u8_to_hex_string(scan_code);
                defmt::error!(
                    "UNABLE TO FIND layout for {} ::: {}",
                    a.as_str(),
                    b.as_str(),
                );
                None
            }
        }
    }
}
