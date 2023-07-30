use std::time::{SystemTime, UNIX_EPOCH};

use super::{
    input::{KbDriverInput, Modifiers},
    kbmap::Key,
};

const MULTI_MODIFIER_KEY_CHAIN_TIMEOUT_MS: u16 = 500;

#[derive(Clone)]
pub struct KbDriverState {
    /// `last_press_epoch_ms` enables multi-modifier key press
    /// combinations by confining successive key presses within
    /// a time window of `MULTI_MODIFIER_KEY_CHAIN_TIMEOUT_MS`.
    /// this prevents `KbDriverState.reset` from being called
    /// when a disparate future key down input is received.
    ///
    /// @BEWARE: this is only ever set iff the received key down
    /// input contains a non-zero modifier. needless to say,
    /// should a key down input contain a bare modifier, then
    /// `last_press_epoch_ms` is not set. however, if such key
    /// down/up input is received **and** its received epoch_ms
    /// exceeds `last_press_epoch_ms`, then `last_press_epoch_ms`
    /// is (re)set back to `None`. pedantically, the corresponding
    /// key up input of a modified key down input is a no-op so
    /// future chaining of modifier key down inputs always presume
    /// the initial modifier key down epoch (SUBJECT TO CHANGE).
    ///
    /// `MULTI_MODIFIER_KEY_CHAIN_TIMEOUT_MS` should be tweaked
    /// to user liking. __not everyone types fast__ **nor** __is
    /// everyone an epik NSA hekkar__.
    pub last_press_epoch_ms: Option<u128>,

    // @TODO{@daveschmenk}: pls fix your client assembly to
    // emit modifier key presses with/without requiring an
    // subsequent ASCII key code. pls.
    pub active_modifiers: Vec<Modifiers>,

    /// because key down scan codes differ from key up scan codes,
    /// we can presume appenditure of additional received differing
    /// key down scan codes and for previous scan codes to be held.
    /// this permits psuedo N-key rollover where N is bound by the
    /// physical hardware limitation of N-key rollover.
    pub active_keys: Vec<u8>,

    pub chained_key_inputs: Vec<KbDriverInput>,
}

impl KbDriverState {
    pub fn reset() -> KbDriverState {
        Self {
            last_press_epoch_ms: None,
            active_modifiers: [].to_vec(),
            active_keys: [].to_vec(),
            chained_key_inputs: [].to_vec(),
        }
    }
    pub fn process_input(&mut self, input: KbDriverInput) {
        let time = SystemTime::now();
        let now = time.duration_since(UNIX_EPOCH).unwrap();
        let now_ms = now.as_millis();

        match input.clone() {
            KbDriverInput::KeyDown((modifier, key_code, _key_character)) => {
                if let Some(last_epoch) = self.last_press_epoch_ms {
                    if now_ms > last_epoch + MULTI_MODIFIER_KEY_CHAIN_TIMEOUT_MS as u128 {
                        self.last_press_epoch_ms = None;
                        self.active_modifiers = [].to_vec();
                        self.active_keys = [].to_vec();
                        self.chained_key_inputs = [].to_vec();
                    }
                } else if modifier.clone().inner() != 0x00 && self.last_press_epoch_ms.is_none() {
                    self.last_press_epoch_ms = Some(now_ms);
                }
                self.active_modifiers.push(modifier);
                self.active_keys.push(key_code);
                self.chained_key_inputs.push(input);
            }
            KbDriverInput::KeyUp((_modifier, _key_code, _key_character)) => {
                if self.last_press_epoch_ms.is_some() {
                    if let Some(last_epoch) = self.last_press_epoch_ms {
                        if now_ms < last_epoch + MULTI_MODIFIER_KEY_CHAIN_TIMEOUT_MS as u128 {
                            // no-op
                        }
                    } else {
                        self.last_press_epoch_ms = None;
                        self.active_modifiers = [].to_vec();
                        self.active_keys = [].to_vec();
                        self.chained_key_inputs = [].to_vec();
                    }
                } else {
                    self.last_press_epoch_ms = None;
                    self.active_modifiers = [].to_vec();
                    self.active_keys = [].to_vec();
                    self.chained_key_inputs = [].to_vec();
                }
            }
        }
    }
    pub fn print(self, key_lookup: &dyn Fn(u8) -> (bool, Option<Key>)) {
        let max = {
            let modifiers = self.active_modifiers.len();
            let keys = self.active_keys.len();

            modifiers.max(keys)
        };
        let mut lines: Vec<String> = [].to_vec();
        for (idx, _) in vec![0; max].iter().enumerate() {
            let modifier = self
                .active_modifiers
                .get(idx)
                .unwrap_or(&Modifiers::Bare(0))
                .clone();
            let key = self.active_keys.get(idx).unwrap_or(&0);

            lines.push(format!(
                "Idx: {} Modifier: {} Key: {:?}",
                idx,
                modifier.outer_as_string(),
                match key {
                    0 => "None".to_string(),
                    _ => {
                        let (_, mapped_key) = key_lookup(*key);
                        if let Some(map) = mapped_key {
                            map.clone().key
                        } else {
                            "None".to_string()
                        }
                    }
                }
            ));
        }
    }
}
