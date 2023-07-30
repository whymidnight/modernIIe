use crate::errors::A2PiError;

use crate::drivers::shared::kb::structs::Key;

// pub const MOD_FN: u8 = 0x80u8;
pub const KEY_ASCII: u8 = 0x7Fu8;

#[derive(Clone)]
pub enum Modifiers {
    Bare(u8),
    OpenApple(u8),
    ClosedApple(u8),
    OpenClosedApple(u8),
}

impl Modifiers {
    pub fn get(modifier_scan_code: u8) -> Option<Modifiers> {
        match modifier_scan_code {
            bare if modifier_scan_code == 0x0u8 => Some(Modifiers::Bare(bare)),
            open if modifier_scan_code == 0x40u8 => Some(Modifiers::OpenApple(open)),
            closed if modifier_scan_code == 0x80u8 => Some(Modifiers::ClosedApple(closed)),
            open_closed if modifier_scan_code == 0xC0u8 => {
                Some(Modifiers::OpenClosedApple(open_closed))
            }
            _ => None,
        }
    }
    pub fn inner(self) -> u8 {
        match self {
            Self::Bare(bare) => bare,
            Self::OpenApple(open) => open,
            Self::ClosedApple(closed) => closed,
            Self::OpenClosedApple(open_closed) => open_closed,
        }
    }
    pub fn outer_as_string(self) -> String {
        match self {
            Self::Bare(_) => "bare".to_string(),
            Self::OpenApple(_) => "open".to_string(),
            Self::ClosedApple(_) => "closed".to_string(),
            Self::OpenClosedApple(_) => "open_closed".to_string(),
        }
    }
}

impl PartialEq for Modifiers {
    fn eq(&self, other: &Self) -> bool {
        let cmp_inner = other.clone().inner();
        match self {
            Self::Bare(bare) => bare == &cmp_inner,
            Self::OpenApple(open) => open == &cmp_inner,
            Self::ClosedApple(closed) => closed == &cmp_inner,
            Self::OpenClosedApple(open_closed) => open_closed == &cmp_inner,
        }
    }
}

#[derive(Clone)]
pub enum KbDriverInput {
    KeyDown((Modifiers, u8, String)),
    KeyUp((Modifiers, u8, String)),
}

impl KbDriverInput {
    pub fn invert(self) -> Self {
        match self {
            Self::KeyUp(input) => Self::KeyDown(input),
            Self::KeyDown(input) => Self::KeyUp(input),
        }
    }
    pub fn from_apple_ii(
        payload: &[u8],
        // `key_mapped` is a closure that @returns truthy if the scan code is
        // mapped to a known key code.
        key_mapped: &dyn Fn(u8) -> (bool, Option<Key>),
    ) -> Result<Option<KbDriverInput>, A2PiError> {
        match payload[0] {
            0x82 => {
                let modifier_got = Modifiers::get(payload[1]);
                if modifier_got.is_none() {
                    return Err(A2PiError::InvalidKBModifier);
                }

                let modifier = modifier_got.unwrap();

                let (is_key_up, mapped_key) = key_mapped(payload[2]);
                if mapped_key.is_none() {
                    return Err(A2PiError::InvalidKBInput);
                }

                let mapped_key_inner = mapped_key.unwrap();

                Ok(match is_key_up {
                    true => Some(KbDriverInput::KeyUp((
                        modifier,
                        payload[2],
                        mapped_key_inner.key,
                    ))),
                    false => Some(KbDriverInput::KeyDown((
                        modifier,
                        payload[2],
                        mapped_key_inner.key,
                    ))),
                })
            }
            _ => Err(A2PiError::InvalidKBPayload),
        }
    }
}
