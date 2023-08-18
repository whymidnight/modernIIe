use core::ops::{Add, Sub};

use alloc::vec;
use alloc::vec::Vec;

use usbd_hid::descriptor::KeyboardReport;

use crate::drivers::shared::kb::Key;

use super::{
    input::{KbDriverInput, KeyEvent, Modifiers},
    kbmap::KeyboardMapEntrant,
    oracle::{KbOracle, KbOracleReports},
};

pub type ActiveKey = (u8, u8, Vec<KeyboardMapEntrant>);
pub type ActiveKeys = Vec<ActiveKey>;

#[derive(Clone)]
pub struct KeyState {
    pub active_keys: ActiveKeys,
    pub oracle: KbOracle,
}

impl KeyState {
    pub fn init() -> Self {
        Self {
            active_keys: Vec::new(),
            oracle: KbOracle::init(),
        }
    }

    pub fn of_single_key(active_key: ActiveKey) -> Self {
        Self {
            active_keys: vec![active_key],
            oracle: KbOracle::init(),
        }
    }

    pub fn clear(&mut self) {
        self.active_keys = Vec::new();
        self.oracle.clear()
    }

    pub fn handle_modifier_event(&mut self, modifier_scan_codes: Vec<Modifiers>) -> Modifiers {
        let layer: u8 = modifier_scan_codes.iter().fold(0u8, |acc, m| {
            let m: u8 = m.clone().into();
            acc + m
        });
        layer.into()
    }

    pub fn handle_key_event(
        &mut self,
        modifier_code: u8,
        key_event_inp: Option<KeyEvent>,
    ) -> (bool, Option<Self>) {
        if let Some(key_event) = key_event_inp {
            let (key, _) = key_event.clone();
            let scan_code = key.key.parse::<u8>().unwrap();
            let active_key = (modifier_code, scan_code, key.usb_hid);

            self.active_keys = vec![active_key.clone()];
            /*
            self.oracle
                .record((modifier_code, key_event.clone()), active_key.clone());
            */

            (true, {
                if modifier_code != 0 && scan_code != 0 {
                    Some(self.clone())
                } else {
                    None
                }
            })
        } else {
            (false, None)
        }
    }

    pub fn last_pressed(&self, modifier_code: u8, scan_code: u8) -> bool {
        match self.active_keys.iter().last() {
            Some(k) => k.0 == modifier_code && k.1 == scan_code,
            None => false,
        }
    }

    pub fn previously_pressed(&self, modifier_code: u8, scan_code: u8) -> bool {
        self.active_keys
            .iter()
            .any(|k| k.0 == modifier_code && k.1 == scan_code)
    }

    pub fn record_key_event(&mut self, modifier_code: u8, scan_code: u8, key_event: KeyEvent) {
        self.oracle.record(
            (modifier_code, key_event.clone()),
            (modifier_code, scan_code, key_event.clone().0.usb_hid),
        );
    }

    pub fn remove_key_event(&mut self, modifier_code: u8, scan_code: u8, key_event: KeyEvent) {
        /*
        self.oracle.remove(
            (modifier_code, key_event.clone()),
            (
                modifier_code.clone(),
                key_event.clone().0.key.parse().unwrap(),
                key_event.clone().0.usb_hid,
            ),
        );
        */
        self.active_keys
            .retain(|k| !(k.0 == modifier_code && k.1 == scan_code))
    }

    pub fn generate_reports(&mut self, for_scan: (Vec<u8>, Vec<u8>)) -> Vec<KbOracleReports> {
        self.oracle.generate_reports(for_scan)
    }
}

impl Add for KeyState {
    type Output = KeyState;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new_self = Self::init();
        new_self.oracle = self.oracle.clone();

        for (_i, left_active_key) in self.active_keys.iter().enumerate() {
            new_self.active_keys.push(left_active_key.clone());
        }
        for (_i, right_active_key) in rhs.active_keys.iter().enumerate() {
            new_self.active_keys.push(right_active_key.clone());

            let (layer, scan_code, keys) = (
                right_active_key.0.clone(),
                right_active_key.1.clone(),
                right_active_key.2.clone(),
            );
            new_self.oracle.record(
                (
                    layer,
                    (
                        Key::define(scan_code, scan_code, keys),
                        KbDriverInput::KeyDown,
                    ),
                ),
                right_active_key.clone(),
            );
        }
        // new_self.active_keys = vec![self.active_keys, rhs.active_keys].concat();
        new_self
    }
}

impl Sub for KeyState {
    type Output = KeyState;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut new_self = Self::init();

        for (_i, left_active_key) in self.active_keys.iter().enumerate() {
            new_self.active_keys.push(left_active_key.clone());
        }
        for (_i, right_active_key) in rhs.active_keys.iter().enumerate() {
            new_self.active_keys.push(right_active_key.clone());
        }
        // new_self.active_keys = vec![self.active_keys, rhs.active_keys].concat();
        new_self
    }
}

impl PartialEq for KeyState {
    fn eq(&self, other: &Self) -> bool {
        if self.active_keys.len() == other.active_keys.len() {
            self.active_keys
                .iter()
                .enumerate()
                .fold(true, |mut acc, (idx, k)| {
                    if other.active_keys[idx].0 != k.0 {
                        acc = false;
                    }
                    if other.active_keys[idx].1 != k.1 {
                        acc = false;
                    }
                    acc = other.active_keys[idx].2.iter().enumerate().fold(
                        true,
                        |mut inner_acc, (inner_idx, other_inner_k)| {
                            if other_inner_k.clone() != k.2[inner_idx] {
                                inner_acc = false;
                            }
                            acc
                        },
                    );
                    acc
                })
        } else {
            false
        }
    }
}
