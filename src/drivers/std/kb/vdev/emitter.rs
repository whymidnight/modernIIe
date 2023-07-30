use std::collections::HashMap;

use itertools::Itertools;

use enigo::KeyboardControllable;

use crate::drivers::kb::{
    input::KbDriverInput,
    state::KbDriverState,
    vdev::{key_codex::VdevKeyMacroSequenceEntrant, state::VdevDeviceKeyEvent},
};

use super::device::VdevDevice;

pub fn vdev_emitter(vdev_device: &mut VdevDevice, kb_driver_input: KbDriverInput) {
    let enigo = &mut vdev_device.enigo;
    match kb_driver_input {
        KbDriverInput::KeyDown((modifier, _key, key_character)) => {
            let vdev_key_got = vdev_device
                .key_codex
                .clone()
                .get_vdev_key(modifier, key_character.clone());
            if vdev_key_got.is_none() {
                println!("\nUNABLE TO FIND KEY CHARACTER CODEX ::: {key_character}\n");
                return;
            }
            let vdev_key = vdev_key_got.unwrap();
            match vdev_key {
                super::key_codex::VdevKey::None(key) => {
                    vdev_device
                        .state
                        .record_key_event(VdevDeviceKeyEvent::record_key_down(key));
                    enigo.key_down(key)
                }
                super::key_codex::VdevKey::Remap(key) => enigo.key_down(key),
                super::key_codex::VdevKey::Macro(macro_seq) => {
                    let mut trace: HashMap<String, VdevKeyMacroSequenceEntrant> = HashMap::new();
                    for (k, entrant) in macro_seq.iter().sorted_by_key(|sk| sk.0) {
                        match trace.clone().get(k) {
                            Some(e) => {
                                if e.until.is_some() {
                                    enigo.key_up(e.clone().into_vdev_key());
                                }
                                if entrant.until.is_some() || entrant.until_after.is_some() {
                                    if let Some(until) = entrant.until.clone() {
                                        trace.insert(until, entrant.clone());
                                    }
                                    if let Some(until_after) = entrant.until_after.clone() {
                                        trace.insert(until_after, entrant.clone());
                                    }
                                    let vdev_key = entrant.clone().into_vdev_key();
                                    vdev_device.state.record_key_event(
                                        VdevDeviceKeyEvent::record_key_down(vdev_key),
                                    );
                                    enigo.key_down(vdev_key);
                                } else {
                                    let vdev_key = entrant.clone().into_vdev_key();
                                    enigo.key_down(vdev_key);
                                    enigo.key_up(vdev_key);
                                }
                                if e.until_after.is_some() {
                                    enigo.key_up(e.clone().into_vdev_key());
                                }
                            }
                            None => {
                                if entrant.until.is_some() || entrant.until_after.is_some() {
                                    if let Some(until) = entrant.until.clone() {
                                        trace.insert(until, entrant.clone());
                                    }
                                    if let Some(until_after) = entrant.until_after.clone() {
                                        trace.insert(until_after, entrant.clone());
                                    }
                                    let vdev_key = entrant.clone().into_vdev_key();
                                    vdev_device.state.record_key_event(
                                        VdevDeviceKeyEvent::record_key_down(vdev_key),
                                    );
                                    enigo.key_down(vdev_key);
                                } else {
                                    enigo.key_click(entrant.clone().into_vdev_key());
                                }
                            }
                        }
                    }
                    vdev_device.clear();
                }
            }
        }
        KbDriverInput::KeyUp((modifier, _key, key_character)) => {
            let vdev_key_got = vdev_device
                .key_codex
                .clone()
                .get_vdev_key(modifier, key_character.clone());
            if vdev_key_got.is_none() {
                println!("\nUNABLE TO FIND KEY CHARACTER CODEX ::: {key_character}\n");
                return;
            }
            let vdev_key = vdev_key_got.unwrap();
            match vdev_key {
                super::key_codex::VdevKey::None(key) => enigo.key_up(key),
                super::key_codex::VdevKey::Remap(key) => enigo.key_up(key),
                super::key_codex::VdevKey::Macro(_key) => {
                    let _ = "unsupported";
                }
            }
        }
    }
}
