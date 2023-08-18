use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport};
use usbd_human_interface_device::device::consumer::MultipleConsumerReport;
use usbd_human_interface_device::page::{Consumer, Keyboard};

use crate::drivers::no_std::kb::input::{KbDriverInput, Modifiers};
use crate::drivers::no_std::kb::kbmap::KeyboardMapEntrant;
use crate::drivers::shared::kb::Key;
use crate::utils;

use super::input::KeyEvent;
use super::state::ActiveKey;

pub type KbOracleTemporalLog = (u8, KeyEvent, ActiveKey);

pub type KbOracleTicket = usize;

#[derive(Clone, Copy)]
pub enum KbOracleReports {
    Keyboard(KeyboardReport),
    Consumer(MediaKeyboardReport),
}

impl KbOracleReports {
    pub fn init() -> KbOracleReports {
        KbOracleReports::Keyboard(KeyboardReport {
            modifier: 0,
            reserved: 0,
            leds: 0,
            keycodes: [0u8; 6],
        })
    }
    pub fn is_empty(&self) -> bool {
        match self {
            KbOracleReports::Keyboard(k) => k.keycodes.iter().all(|&key_code| key_code == 0),
            KbOracleReports::Consumer(c) => c.usage_id == 0,
        }
    }
}

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
        let recordable = true;

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

        let mut idx = 0;
        let temporal_logs = self.temporal_logs.clone();

        let mut new_cycle = Vec::new();
        self.temporal_logs.iter().for_each(|active_key| {
            let (_active_ticket, (active_layer, key, _keys)) = active_key.clone();

            // basically we attest a multi-modifier against all the other key presses
            // by evaluating for any/all previous key presses of which are of the bare
            // layer (modifier_code) then hereinstating as skipped when generating
            // `Vec<KeyboardReport>`.

            if is_macro_key && layer == active_layer && key.0.key.parse::<u8>().unwrap() == 0x00 {
                idx += 1;
                // new_cycle.push(active_key.clone());
            } else if layer == active_layer {
                if key_event.clone().0.key == key.0.key {
                    // recordable = false;
                    if idx > 0 {
                        let last = &temporal_logs.as_slice()[idx - 1];
                        let (_a, (_b, last_key, _c)) = last.clone();
                        if last_key.0.key != key_event.clone().0.key {
                            defmt::error!(
                                "RENEWING due to dissimilar last REPEATING KEY !!! {} {}",
                                key.0.key.as_str(),
                                last_key.0.key.as_str(),
                            );
                            let is_modifier = Modifiers::get(active_layer);
                            if is_modifier.is_some() {
                                new_cycle.push(active_key.clone());
                            }
                        } else {
                            defmt::error!("SKIPPING due to same key !!! {}", key.0.key.as_str());
                        }
                        idx += 1;
                        // new_cycle.push(last.clone());
                    } else {
                        idx += 1;
                        defmt::error!("ELSE SKIPPING due to same key !!! {}", key.0.key.as_str());
                    }
                    new_cycle.push(active_key.clone());
                    // break;
                } else {
                    defmt::error!(
                        "ELSE SKIP !!! {} {} {}",
                        utils::hex::u8_to_hex_string(layer).as_str(),
                        key_event.clone().0.key.as_str(),
                        key.0.key.as_str()
                    );
                    new_cycle.push(temporal_log.clone());
                    new_cycle.push(active_key.clone());
                    /*
                     */
                    idx += 1;
                }
            } else {
                defmt::error!("PERMITTING !!! {} {}", active_layer, key.0.key.as_str());
                idx += 1;
            }
        });
        if !new_cycle.is_empty() {
            self.clear();
            self.temporal_logs = new_cycle;
            self.temporal_logs = self.temporal_logs.iter().fold(vec![], |mut acc, k| {
                let (_a, (_b, key, _c)) = k;
                if acc.iter().any(|x| {
                    let (_aa, (_bb, acc_key, _cc)) = x;
                    if acc_key.0.key == key.0.key {
                        return true;
                    }
                    false
                }) {
                    acc.push(k.clone());
                    return acc;
                }
                acc
            });
            self.temporal_logs.reverse()
        } else {
            idx = 0;
            self.temporal_logs.retain(|active_key| {
                let (_active_ticket, (active_layer, key, _keys)) = active_key.clone();

                // basically we attest a multi-modifier against all the other key presses
                // by evaluating for any/all previous key presses of which are of the bare
                // layer (modifier_code) then hereinstating as skipped when generating
                // `Vec<KeyboardReport>`.

                if is_macro_key && layer == active_layer && key.0.key.parse::<u8>().unwrap() == 0x00
                {
                    idx += 1;
                    false
                } else if layer == active_layer {
                    if key_event.clone().0.key == key.0.key {
                        // recordable = false;
                        if idx > 0 {
                            let (_a, (_b, last_key, _c)) = &temporal_logs.as_slice()[idx - 1];
                            if last_key.0.key != key_event.clone().0.key {
                                defmt::error!(
                                    "RENEWING due to dissimilar last REPEATING KEY !!! {} {}",
                                    key.0.key.as_str(),
                                    last_key.0.key.as_str(),
                                );
                            } else {
                                defmt::error!(
                                    "0 SKIPPING due to same key !!! {}",
                                    key.0.key.as_str()
                                );
                            }
                            idx += 1;
                            false
                        } else {
                            idx += 1;
                            defmt::error!("1 SKIPPING due to same key !!! {}", key.0.key.as_str());
                            false
                        }
                        // break;
                    } else {
                        defmt::error!(
                            "ELSE SKIP !!! {} {} {}",
                            utils::hex::u8_to_hex_string(layer).as_str(),
                            key_event.clone().0.key.as_str(),
                            key.0.key.as_str()
                        );
                        /*
                         */
                        idx += 1;
                        true
                    }
                } else {
                    defmt::error!("PERMITTING !!! {} {}", active_layer, key.0.key.as_str());
                    idx += 1;
                    true
                }
            });
        }
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

    pub fn drop_index(&mut self, index: usize) {
        let mut idx = 0usize;
        self.temporal_logs.retain(|_| {
            if idx == index {
                return false;
            }
            idx += 1;
            true
        });
    }

    pub fn drop_indexes(&mut self, indexes: Vec<usize>) {
        indexes.iter().for_each(|&idx| self.drop_index(idx))
    }

    pub fn clear(&mut self) {
        self.temporal_logs.clear();
        self.current_ticket = 0;
    }

    pub fn generate_reports(&mut self, for_scan: (Vec<u8>, Vec<u8>)) -> Vec<KbOracleReports> {
        defmt::info!(
            "FOR SCAN !!! {} {}",
            for_scan
                .0
                .clone()
                .iter()
                .map(|&k| k.into())
                .collect::<Vec<u8>>()
                .iter()
                .map(|&k| utils::hex::u8_to_hex_string(k))
                .collect::<Vec<String>>()
                .iter()
                .map(|m| m.as_str())
                .collect::<Vec<&str>>()
                .as_slice(),
            for_scan
                .1
                .clone()
                .iter()
                .map(|&k| k.into())
                .collect::<Vec<u8>>()
                .iter()
                .map(|&k| utils::hex::u8_to_hex_string(k))
                .collect::<Vec<String>>()
                .iter()
                .map(|m| m.as_str())
                .collect::<Vec<&str>>()
                .as_slice(),
        );
        self.temporal_logs.reverse();
        let keyboard_log = self
            .temporal_logs
            .iter()
            .filter(|l| {
                let log = l.1 .2 .2.clone();
                let key = &l.1 .2;
                let consumer_entrants = log
                    .iter()
                    .filter(|entrant| match entrant {
                        KeyboardMapEntrant::Keyboard(_) => false,
                        KeyboardMapEntrant::Consumer(c) => {
                            let cons: u16 = c.clone().into();
                            defmt::error!("[KLOG]: _--------_ IS CONSUMER {}", cons);
                            true
                        }
                    })
                    .cloned()
                    .collect::<Vec<KeyboardMapEntrant>>();
                defmt::error!(
                    "!!!!!!!!!!!! !!! {} {} {}",
                    consumer_entrants.is_empty(),
                    key.0,
                    key.1
                );

                consumer_entrants.is_empty()
            })
            .cloned()
            .collect::<Vec<(
                usize,
                (u8, (Key, KbDriverInput), (u8, u8, Vec<KeyboardMapEntrant>)),
            )>>()
            .iter()
            .fold(Vec::new(), |mut acc, l| {
                let key = &l.1 .2;
                let is_for_scan_mods = for_scan.0.contains(&key.0);
                let is_for_scan_chars = for_scan.1.contains(&key.1);
                defmt::error!(
                    "???????????? !!! {} {} {} {} {}",
                    self.skipped_tickets.contains(&l.clone().0),
                    is_for_scan_mods,
                    is_for_scan_chars,
                    key.0,
                    key.1
                );
                if !self.skipped_tickets.contains(&l.clone().0)
                // && (is_for_scan_mods || is_for_scan_chars)
                {
                    defmt::error!(
                        "LOGGING_REPORT !!! {} {} {} {} {}",
                        self.skipped_tickets.contains(&l.clone().0),
                        is_for_scan_mods,
                        is_for_scan_chars,
                        key.0,
                        key.1
                    );
                    acc.push(l);
                } else {
                    defmt::error!(
                        "SKIPPING_REPORT due to same key !!! {} {} {} {} {}",
                        self.skipped_tickets.contains(&l.clone().0),
                        is_for_scan_mods,
                        is_for_scan_chars,
                        key.0,
                        key.1
                    );
                }
                acc
            })
            .iter()
            .map(|l| l.1 .2 .2.clone())
            .collect::<Vec<Vec<KeyboardMapEntrant>>>()
            .iter()
            .flat_map(|keys| keys.clone())
            .collect::<Vec<KeyboardMapEntrant>>();

        let consumer_log = self
            .temporal_logs
            .iter()
            .filter(|l| {
                let log = l.1 .2 .2.clone();
                let consumer_entrants = log
                    .iter()
                    .filter(|entrant| match entrant {
                        KeyboardMapEntrant::Keyboard(_) => false,
                        KeyboardMapEntrant::Consumer(c) => {
                            let cons: u16 = c.clone().into();
                            defmt::error!("[CLOG]: _--------_ IS CONSUMER {}", cons);
                            true
                        }
                    })
                    .cloned()
                    .collect::<Vec<KeyboardMapEntrant>>();

                !consumer_entrants.is_empty()
            })
            .cloned()
            .collect::<Vec<(
                usize,
                (u8, (Key, KbDriverInput), (u8, u8, Vec<KeyboardMapEntrant>)),
            )>>()
            .iter()
            .fold(Vec::new(), |mut acc, l| {
                let key = &l.1 .2;
                let is_for_scan_mods = for_scan.0.contains(&key.0);
                let is_for_scan_chars = for_scan.1.contains(&key.1);
                if !self.skipped_tickets.contains(&l.clone().0)
                // && (is_for_scan_mods || is_for_scan_chars)
                {
                    defmt::error!(
                        "LOGGING_REPORT !!! {} {} {} {} {}",
                        self.skipped_tickets.contains(&l.clone().0),
                        is_for_scan_mods,
                        is_for_scan_chars,
                        key.0,
                        key.1
                    );
                    acc.push(l);
                } else {
                    defmt::error!(
                        "SKIPPING_REPORT due to same key !!! {} {} {} {} {}",
                        self.skipped_tickets.contains(&l.clone().0),
                        is_for_scan_mods,
                        is_for_scan_chars,
                        key.0,
                        key.1
                    );
                }
                acc
            })
            .iter()
            .map(|l| l.1 .2 .2.clone())
            .collect::<Vec<Vec<KeyboardMapEntrant>>>()
            .iter()
            .flat_map(|keys| keys.clone())
            .collect::<Vec<KeyboardMapEntrant>>();

        let mut reports: Vec<KbOracleReports> = Vec::new();

        for k in vec![keyboard_log].iter() {
            let keys = k;
            let report = KeyboardReport {
                modifier: {
                    keys.iter()
                        .fold(Vec::new() as Vec<u8>, |mut acc, k| {
                            let k_u8: u8 = k.clone().into();
                            let modded = match k_u8 {
                                _ if k_u8 == Keyboard::LeftControl.into() => 0,
                                _ if k_u8 == Keyboard::LeftShift.into() => 1,
                                _ if k_u8 == Keyboard::LeftAlt.into() => 2,
                                _ if k_u8 == Keyboard::LeftGUI.into() => 3,
                                _ if k_u8 == Keyboard::RightControl.into() => 4,
                                _ if k_u8 == Keyboard::RightShift.into() => 5,
                                _ if k_u8 == Keyboard::RightAlt.into() => 6,
                                _ if k_u8 == Keyboard::RightGUI.into() => 7,
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
                    for (idx, key) in keys.clone().iter().enumerate() {
                        if idx >= 6 {
                            break;
                        }
                        keycodes[idx] = key.clone().into();
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

            */

            defmt::info!("reporting !!! {} {}", report.modifier, report.keycodes);
            reports.push(KbOracleReports::Keyboard(report))
        }

        for c in vec![consumer_log].iter() {
            if c.is_empty() {
                break;
            }

            let usage_id = c[0].clone().into();
            let report = MediaKeyboardReport { usage_id };
            /*
            defmt::info!(
                "keys chunked :::: {}",
                keys.iter()
                    .map(|&k| k.into())
                    .collect::<Vec<u8>>()
                    .as_slice()
            );

            */

            defmt::info!("reporting !!! {}", usage_id.clone());
            reports.push(KbOracleReports::Consumer(report))
        }

        reports = reports
            .clone()
            .iter()
            .filter(|report| !report.is_empty())
            .cloned()
            .collect::<Vec<KbOracleReports>>();

        defmt::info!("FINAL !!! {} {}", self.temporal_logs.len(), reports.len());

        // self.clear();
        reports
    }
}
