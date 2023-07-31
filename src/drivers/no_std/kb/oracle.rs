use alloc::vec;
use alloc::vec::Vec;
use usbd_hid::descriptor::KeyboardReport;
use usbd_human_interface_device::page::Keyboard;

use crate::utils;

use super::input::KeyEvent;
use super::state::ActiveKey;

pub type KbOracleTemporalLog = (u8, KeyEvent, ActiveKey);

pub type KbOracleTicket = usize;

/*
  we must omit a former modifier keycode from the `KeyboardReport` as to correctly
  report macro keys in the next `KeyboardReport` wherein both the modifier and character
  scan codes have an existant `crate::drivers::no_std::kb::kbmap::LayoutKeyWithHIDEntrant`
  in `self.key_map.layout` instanced via [hoist_hid_keyboard_map](src/drivers/no_std/kb/kbmap/hid.rs).

  observation:

  | initial scan | current scan |
  |--------------|--------------|
  | (0x80, 0x00) | (0x80, 0x01) |

  broken psuedo-report:

  | initial report     | current report     |
  |--------------------|--------------------|
  | (0x80, 0x00, 0x00) | (0x80, 0x80, 0x01) |

  correct psuedo-report:

  | initial report     | current report     |
  |--------------------|--------------------|
  | (0x80, 0x00, 0x00) | (0x80, 0x01, 0x00) |

  ---

  niave me believes we can just always assume this would ever
  happen where the former modifier key code index is zero.
  my train of thought was to negate the effect where the bare
  modifier was being _held_ whilst activating the macro keys.

match &modified {
    Some(input_event) => {
        // we can suppose that if
        if k.0 == layer && &KeyState::of_single_key(k.clone()) == input_event {
            false
        } else {
            true
        }
    }
    None => modifier_scan_codes.clone().contains(&k.0),
}
*/
#[derive(Clone)]
pub struct KbOracle {
    // so everytime we mutate `KeyState` this gets called to record
    // `KeyState.active_keys` - then we index that on the modifier and
    // character scan codes giving us chronological expression of handled
    // key presses. this is advantageous since it would tolerate disordered
    // modifier key press sequences thus correctly rendering the multi-modifier
    // macro.
    pub temporal_logs: Vec<(KbOracleTicket, KbOracleTemporalLog)>,
    pub skipped_tickets: Vec<KbOracleTicket>,
    pub current_ticket: KbOracleTicket,
}

impl KbOracle {
    pub fn init() -> Self {
        Self {
            temporal_logs: Vec::new(),
            skipped_tickets: Vec::new(),
            current_ticket: 0,
        }
    }

    pub fn record(&mut self, key_record: (u8, KeyEvent), temporal_log: ActiveKey) {
        let ticket = self.temporal_logs.len() as KbOracleTicket;
        let mut recordable = true;

        let (layer, key_event) = (key_record.0, key_record.1);
        let is_macro_key = {
            if key_event.0.key.parse::<u8>().unwrap() == layer
                && layer != 0
                && temporal_log.1 != 0x00
            {
                true
            } else {
                false
            }
        };
        defmt::error!(
            "recording handle {} :: {}",
            utils::hex::u8_to_hex_string(layer).as_str(),
            key_event.0.key.as_str()
        );

        self.temporal_logs.retain(|active_key| {
            let (active_ticket, (active_layer, key, _keys)) = active_key.clone();

            // basically we attest a multi-modifier against all the other key presses
            // by evaluating for any/all previous key presses of which are of the bare
            // layer (modifier_code) then hereinstating as skipped when generating
            // `Vec<KeyboardReport>`.

            if is_macro_key && layer == active_layer && key.0.key.parse::<u8>().unwrap() == 0x00 {
                false
            } else if layer == active_layer {
                if key_event.clone().0.key == key.0.key {
                    // recordable = false;
                    // defmt::error!("SKIPPING due to same key !!!");
                    false
                    // break;
                } else {
                    /*
                    defmt::error!(
                        "ELSE SKIP !!! {} {} {}",
                        utils::hex::u8_to_hex_string(layer).as_str(),
                        key_event.clone().0.key.as_str(),
                        key.0.key.as_str()
                    );
                    */
                    true
                }
            } else {
                // defmt::error!("PERMITTING !!! {} {}", active_layer, key.0.key.as_str());
                true
            }
        });
        /*
         */

        if recordable || is_macro_key {
            // defmt::info!("PUSHING TEMP LOG !!!");
            self.temporal_logs
                .push((ticket, (layer, key_event, temporal_log)))
        }
    }

    pub fn remove(&mut self, key_record: (u8, KeyEvent), temporal_log: ActiveKey) {
        let (layer, key_event) = (key_record.0, key_record.1);
        defmt::error!(
            "removing handle {} :: {}",
            utils::hex::u8_to_hex_string(layer).as_str(),
            key_event.0.key.as_str()
        );
        self.temporal_logs.retain(|active_key| {
            let (active_ticket, (active_layer, key, _keys)) = active_key.clone();

            // basically we attest a multi-modifier against all the other key presses
            // by evaluating for any/all previous key presses of which are of the bare
            // layer (modifier_code) then hereinstating as skipped when generating
            // `Vec<KeyboardReport>`.

            if layer == active_layer && key_event.clone().0.key == key.0.key {
                false
            } else {
                true
            }
        });
        /*
         */
    }

    pub fn drop(&mut self, ticket: KbOracleTicket) {
        self.temporal_logs.retain(|active_key| {
            let (active_ticket, (_active_layer, _key, _keys)) = active_key.clone();

            active_ticket != ticket
        });
        /*
         */
    }

    pub fn clear(&mut self) {
        self.temporal_logs.clear();
        self.current_ticket = 0;
    }

    pub fn generate_reports(&mut self, for_scan: (Vec<u8>, Vec<u8>)) -> Vec<KeyboardReport> {
        let log = self
            .temporal_logs
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut acc, (idx, l)| {
                // &&
                let key = &l.1 .2;
                let is_for_scan_mods = for_scan.0.contains(&key.0) == true;
                let is_for_scan_chars = for_scan.1.contains(&key.1) == true;
                if !self.skipped_tickets.contains(&l.clone().0)
                    && (is_for_scan_mods || is_for_scan_chars)
                {
                    acc.push(l);
                } else {
                    defmt::error!("SKIPPING_REPORT due to same key !!! {} {}", key.0, key.1);
                }
                /*
                if idx >= self.current_ticket {
                }
                    */
                acc
            })
            .iter()
            .map(|l| l.1 .2 .2.clone())
            .collect::<Vec<Vec<Keyboard>>>()
            .iter()
            .flat_map(|keys| keys.clone())
            .collect::<Vec<Keyboard>>();
        let log_chunks = vec![log.clone()];

        let mut reports = Vec::new();
        for k in log_chunks.iter() {
            let keys = k;
            let report = KeyboardReport {
                modifier: {
                    keys.iter()
                        .fold(Vec::new() as Vec<u8>, |mut acc, k| {
                            let modded = match k {
                                Keyboard::LeftControl => 0,
                                Keyboard::LeftShift => 1,
                                Keyboard::LeftAlt => 2,
                                Keyboard::LeftGUI => 3,
                                Keyboard::RightControl => 4,
                                Keyboard::RightShift => 5,
                                Keyboard::RightAlt => 6,
                                Keyboard::RightGUI => 7,
                                _ => 8,
                            };
                            acc.push(modded);
                            acc
                        })
                        .iter()
                        .fold(Vec::new() as Vec<u8>, |mut acc, &k| {
                            if !acc.contains(&k) && k < 8 {
                                acc.push(k)
                            }
                            acc
                        })
                        .iter()
                        .fold(0u8, |acc, &c| {
                            let modded = match c {
                                0 => 1u8 << 0u8,
                                1 => 1 << 1,
                                2 => 1 << 2,
                                3 => 1 << 3,
                                4 => 1 << 4,
                                5 => 1 << 5,
                                6 => 1 << 6,
                                7 => 1 << 7,
                                _ => 0,
                            };

                            acc | modded
                        })
                },
                reserved: 0,
                leds: 0,
                keycodes: {
                    let mut keycodes: [u8; 6] = [0x0u8; 6];
                    for (idx, &key) in keys.iter().enumerate() {
                        if idx >= 6 {
                            break;
                        }
                        keycodes[idx] = key.into();
                    }

                    keycodes
                },
            };
            /*
            defmt::info!(
                "keys chunked :::: {}",
                keys.iter()
                    .map(|&k| k.into())
                    .collect::<Vec<u8>>()
                    .as_slice()
            );

            defmt::info!("reporting !!! {} {}", report.modifier, report.keycodes);
            */

            reports.push(report)
        }
        defmt::info!(
            "FINAL !!! {} {} {}",
            self.temporal_logs.len(),
            log.clone()
                .iter()
                .map(|&k| k.into())
                .collect::<Vec<u8>>()
                .as_slice(),
            reports.len()
        );

        // self.clear();
        reports
    }
}
