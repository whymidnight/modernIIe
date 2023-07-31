use core::convert::Infallible;

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use cortex_m::delay::Delay;
use embedded_hal::digital::v2::InputPin;
use usbd_hid::descriptor::KeyboardReport;
use usbd_human_interface_device::page::Keyboard;

use crate::{drivers::shared::kb::*, utils};

use super::{
    decoder::{Debounce, KeyScan, NUM_COLS, NUM_MODS, NUM_ROWS},
    input::Modify,
    input::ModifyEvent,
    kbmap::KeyMap,
    state::KeyState,
};

#[derive(Clone)]
pub struct KbDriver {
    pub key_map: KeyMap,
    pub key_state: KeyState,
}

impl KeyboardDriver for KbDriver {
    fn init() -> KbDriver {
        KbDriver {
            key_map: KeyMap::init(),
            key_state: KeyState::init(),
        }
    }

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
    ) -> Option<Vec<KeyboardReport>> {
        let mut key_state = KeyState::init();

        let key_scan = KeyScan::scan(modifiers, rows, columns, delay, debounce);
        let (modifiers, characters) = key_scan.into_decoder();
        let (modifier_scan_codes, character_scan_codes): (Vec<u8>, Vec<u8>) =
            (modifiers.into(), characters.into());

        /*

        defmt::info!(
            "{} :: {}",
            modifier_scan_codes
                .iter()
                .map(|k| utils::hex::u8_to_hex_string(*k))
                .collect::<Vec<String>>()
                .iter()
                .map(|h| h.as_str())
                .collect::<Vec<&str>>()
                .as_slice(),
            character_scan_codes
                .iter()
                .map(|k| utils::hex::u8_to_hex_string(*k))
                .collect::<Vec<String>>()
                .iter()
                .map(|h| h.as_str())
                .collect::<Vec<&str>>()
                .as_slice()
        );
         */

        if !modifier_scan_codes.is_empty() || !character_scan_codes.is_empty() {
            let layer: u8 = key_state
                .handle_modifier_event(modifier_scan_codes.iter().map(|&m| m.into()).collect())
                .into();

            let modified: Modify = {
                let scan_code = 0x00;
                let key_event_input = self.key_map.clone().find_input(layer, scan_code);
                let (handled, modified_active_keys) =
                    key_state.handle_key_event(layer, key_event_input.clone());
                if !self.key_state.previously_pressed(layer, scan_code) {
                    if !handled {
                        defmt::error!(
                            "unable to handle {} :: {}",
                            utils::hex::u8_to_hex_string(layer).as_str(),
                            utils::hex::u8_to_hex_string(scan_code).as_str()
                        );
                        Modify::None
                    } else {
                        if let Some(event) = key_event_input.clone() {
                            Modify::Record((layer, scan_code, event.clone()))
                        } else {
                            Modify::None
                        }
                    }
                } else {
                    if let Some(event) = key_event_input {
                        Modify::Remove((layer, scan_code, event.clone()))
                    } else {
                        Modify::None
                    }
                }
            };
            /*
             */

            defmt::error!(
                "{} ------------------- {} --------------------",
                layer,
                modifier_scan_codes
                    .iter()
                    .map(|&m| utils::hex::u8_to_hex_string(m))
                    .collect::<Vec<String>>()
                    .iter()
                    .map(|m| m.as_str())
                    .collect::<Vec<&str>>()
                    .as_slice()
            );
            if character_scan_codes.is_empty() {
                match &modified {
                    Modify::Record((modify_layer, modify_scan_code, modify_event)) => {
                        let dropping = self.key_state.oracle.temporal_logs.iter().fold(
                            Vec::new(),
                            |mut acc, l| {
                                if l.1 .0 == layer {
                                    acc.push(l.0);
                                }
                                acc
                            },
                        );
                        defmt::error!(
                            "00 BBBB222222 ::: {} ---- {}",
                            dropping.as_slice(),
                            self.key_state
                                .oracle
                                .temporal_logs
                                .iter()
                                .map(|m| utils::hex::u8_to_hex_string(m.1 .0))
                                .collect::<Vec<String>>()
                                .iter()
                                .map(|m| m.as_str())
                                .collect::<Vec<&str>>()
                                .as_slice()
                        );
                        for drop in dropping {
                            self.key_state.oracle.drop(drop);
                        }

                        let a = modify_event.clone();
                        self.key_state.record_key_event(
                            modify_layer.clone(),
                            modify_scan_code.clone(),
                            // @CHECK::: does this finally overwrite bare
                            // and render the keyboard vec from our declared hid map?
                            a,
                        );
                    }
                    Modify::Remove((modify_layer, modify_scan_code, modify_event)) => {
                        defmt::error!("00 CCCC333333");
                        let a = modify_event.clone();
                        self.key_state.remove_key_event(
                            modify_layer.clone(),
                            modify_scan_code.clone(),
                            a,
                        );
                    }
                    Modify::None => {
                        defmt::error!("00 DDDD444444");
                    }
                }
            }
            for scan_code in character_scan_codes.clone() {
                let mut character_layer = layer;
                let mut key_event_input =
                    self.key_map.clone().find_input(character_layer, scan_code);

                if key_event_input.is_none() {
                    character_layer = 0x00;
                    key_event_input = self.key_map.clone().find_input(character_layer, scan_code);
                }

                let (handled, handled_modified) =
                    key_state.handle_key_event(character_layer, key_event_input.clone());

                match key_event_input.clone() {
                    Some(event_input) => match &modified {
                        Modify::Record((modify_layer, modify_scan_code, modify_event)) => {
                            if layer != 0 && event_input.0.key.parse::<u8>().unwrap() != 0x00 {
                                defmt::error!("11 AAAA111111");
                                let a = modify_event.clone();
                                if handled_modified.is_some() {
                                    self.key_state.oracle.remove(
                                        (modify_layer.clone(), a.clone()),
                                        (
                                            modify_layer.clone(),
                                            a.clone().0.key.parse().unwrap(),
                                            a.clone().0.usb_hid,
                                        ),
                                    );
                                }
                                self.key_state.remove_key_event(
                                    modify_layer.clone(),
                                    modify_scan_code.clone(),
                                    // @CHECK::: does this finally overwrite bare modifiers
                                    // and render the keyboard vec from our declared hid map?
                                    a,
                                );
                            } else if layer != 0 && event_input.0.key.parse::<u8>().unwrap() == 0x00
                            {
                                // need to garbage collect a former modifier with key code
                                if handled_modified.is_none() {
                                    defmt::error!(
                                        "11 BBBB222222 {}",
                                        self.key_state.oracle.temporal_logs.iter().fold(
                                            0,
                                            |mut acc, l| {
                                                if l.1 .0 == 0 {
                                                    acc = acc + 1;
                                                }
                                                acc
                                            }
                                        )
                                    );
                                    let a = modify_event.clone();
                                    self.key_state.record_key_event(
                                        modify_layer.clone(),
                                        modify_scan_code.clone(),
                                        // @CHECK::: does this finally overwrite bare modifiers
                                        // and render the keyboard vec from our declared hid map?
                                        a,
                                    );
                                }
                                /*
                                {
                                                                let a = modify_event.clone();
                                                                self.key_state.record_key_event(
                                                                    modify_layer.clone(),
                                                                    modify_scan_code.clone(),
                                                                    a,
                                                                );
                                                            }
                                                                 */
                            }
                        }
                        Modify::Remove((modify_layer, modify_scan_code, modify_event)) => {
                            defmt::error!("11 CCCC333333");
                            let a = modify_event.clone();
                            self.key_state.remove_key_event(
                                modify_layer.clone(),
                                modify_scan_code.clone(),
                                a,
                            );
                        }
                        Modify::None => {
                            defmt::error!("11 DDDD444444");
                        }
                    },
                    None => {}
                }

                if !self
                    .key_state
                    .previously_pressed(character_layer, scan_code)
                {
                    if !handled {
                        defmt::error!(
                            "unable to handle {} :: {}",
                            utils::hex::u8_to_hex_string(character_layer).as_str(),
                            utils::hex::u8_to_hex_string(scan_code).as_str()
                        );
                    } else {
                        if let Some(event) = key_event_input {
                            defmt::error!(
                                "recording char handle {} :: {}",
                                utils::hex::u8_to_hex_string(character_layer).as_str(),
                                utils::hex::u8_to_hex_string(scan_code).as_str()
                            );
                            self.key_state
                                .record_key_event(character_layer, scan_code, event)
                        }
                    }
                } else {
                    if let Some(event) = key_event_input {
                        defmt::error!(
                            "removing char handle {} :: {}",
                            utils::hex::u8_to_hex_string(character_layer).as_str(),
                            utils::hex::u8_to_hex_string(scan_code).as_str()
                        );
                        self.key_state
                            .remove_key_event(character_layer, scan_code, event)
                    }
                }
            }

            /*
            defmt::info!(
                "[A]: \n{}\n",
                self.key_state
                    .active_keys
                    .iter()
                    .map(|k| utils::hex::u8_to_hex_string(k.1))
                    .collect::<Vec<String>>()
                    .iter()
                    .map(|h| h.as_str())
                    .collect::<Vec<&str>>()
                    .as_slice(),
            );
            defmt::info!(
                "[B]: \n{}\n",
                key_state
                    .active_keys
                    .iter()
                    .map(|k| utils::hex::u8_to_hex_string(k.1))
                    .collect::<Vec<String>>()
                    .iter()
                    .map(|h| h.as_str())
                    .collect::<Vec<&str>>()
                    .as_slice(),
            );
            */
            /*
            self.key_state = self.key_state.clone() + key_state;
            // finally remove any keys that werent seen in this key scan
            self.key_state.active_keys.retain(|k| match k.0 {
                0 => character_scan_codes.clone().contains(&k.1),
                _ => modifier_scan_codes.clone().contains(&k.0),
            });
            */
            /*
            defmt::info!(
                "{} -- [C::mods]: \n{}\n",
                layer,
                self.key_state
                    .active_keys
                    .iter()
                    .map(|k| utils::hex::u8_to_hex_string(k.0))
                    .collect::<Vec<String>>()
                    .iter()
                    .map(|h| h.as_str())
                    .collect::<Vec<&str>>()
                    .as_slice(),
            );
            defmt::info!(
                "[C::chars]: \n{}\n",
                self.key_state
                    .active_keys
                    .iter()
                    .map(|k| utils::hex::u8_to_hex_string(k.1))
                    .collect::<Vec<String>>()
                    .iter()
                    .map(|h| h.as_str())
                    .collect::<Vec<&str>>()
                    .as_slice(),
            );

            */

            Some(self.key_state.generate_reports((
                vec![modifier_scan_codes, vec![layer]].concat(),
                character_scan_codes,
            )))
        } else {
            //defmt::info!("clearing keyboard report!!!");
            self.key_state.clear();
            Some(vec![KeyboardReport {
                modifier: 0,
                reserved: 0,
                leds: 0,
                keycodes: [0x0u8; 6],
            }])
        }
    }

    fn hid_report(self) -> Vec<KeyboardReport> {
        let mut reports: Vec<KeyboardReport> = Vec::new();
        reports
    }
}
