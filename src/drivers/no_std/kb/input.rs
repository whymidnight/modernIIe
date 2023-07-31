use crate::{drivers::shared::kb::Key, utils};
use alloc::string::*;

// pub const MOD_FN: u8 = 0x80u8;
pub const KEY_ASCII: u8 = 0x7Fu8;

pub type KeyEvent = (Key, KbDriverInput);

pub type ModifyEvent = (u8, u8, KeyEvent);
#[derive(Clone)]
pub enum Modify {
    Record(ModifyEvent),
    Remove(ModifyEvent),
    None,
}

#[derive(Clone)]
#[repr(u8)]
pub enum Modifiers {
    Bare = 0x0u8,

    OpenApple = 0x40,              // SW0
    ClosedApple = 0x80,            // SW1
    OpenClosedApple = 0x40 + 0x80, // SW0 + SW1
    Control = 0x01,                // Control
    Reset = 0x02,                  // Reset
    Shift = 0x04,                  // Shift

    // Open Apple Combined
    OpenAppleControl = 0x40 + 0x01,                         // SW0
    OpenAppleReset = 0x40 + 0x02,                           // SW0
    OpenAppleResetControl = 0x40 + 0x02 + 0x01,             // SW0
    OpenAppleShift = 0x40 + 0x04,                           // SW0
    OpenAppleShiftReset = 0x40 + 0x04 + 0x02,               // SW0
    OpenAppleShiftResetControl = 0x40 + 0x04 + 0x02 + 0x01, // SW0

    // Closed Apple Combined
    ClosedAppleControl = 0x80 + 0x01,                         // SW0
    ClosedAppleReset = 0x80 + 0x02,                           // SW0
    ClosedAppleResetControl = 0x80 + 0x02 + 0x01,             // SW0
    ClosedAppleShift = 0x80 + 0x04,                           // SW0
    ClosedAppleShiftReset = 0x80 + 0x04 + 0x02,               // SW0
    ClosedAppleShiftResetControl = 0x80 + 0x04 + 0x02 + 0x01, // SW0

    // Open + Closed Apple Combined
    OpenClosedAppleControl = 0x40 + 0x80 + 0x01, // SW0
    OpenClosedAppleReset = 0x40 + 0x80 + 0x02,   // SW0
    OpenClosedAppleResetControl = 0x40 + 0x80 + 0x02 + 0x01, // SW0
    OpenClosedAppleShift = 0x40 + 0x80 + 0x04,   // SW0
    OpenClosedAppleShiftReset = 0x40 + 0x80 + 0x04 + 0x02, // SW0
    OpenClosedAppleShiftResetControl = 0x40 + 0x80 + 0x04 + 0x02 + 0x01, // SW0

    // Control Combined
    ControlReset = 0x01 + 0x02,
    ControlResetShift = 0x01 + 0x02 + 0x04,
    ControlShift = 0x01 + 0x04,
}

impl Modifiers {
    pub fn get(modifier_scan_code: u8) -> Option<Modifiers> {
        match modifier_scan_code {
            _ if modifier_scan_code == 0x0u8 => Some(Modifiers::Bare),
            _ if modifier_scan_code == Modifiers::OpenApple.into() => Some(Modifiers::OpenApple),
            _ if modifier_scan_code == Modifiers::ClosedApple.into() => {
                Some(Modifiers::ClosedApple)
            }
            _ if modifier_scan_code == Modifiers::OpenClosedApple.into() => {
                Some(Modifiers::OpenClosedApple)
            }
            _ if modifier_scan_code == Modifiers::Control.into() => Some(Modifiers::Control),
            _ if modifier_scan_code == Modifiers::Reset.into() => Some(Modifiers::Reset),
            _ if modifier_scan_code == Modifiers::Shift.into() => Some(Modifiers::Shift),

            // multi-modifier
            // open apple
            _ if modifier_scan_code == Modifiers::OpenAppleControl.into() => {
                Some(Modifiers::OpenAppleControl)
            }
            _ if modifier_scan_code == Modifiers::OpenAppleReset.into() => {
                Some(Modifiers::OpenAppleReset)
            }
            _ if modifier_scan_code == Modifiers::OpenAppleResetControl.into() => {
                Some(Modifiers::OpenAppleResetControl)
            }
            _ if modifier_scan_code == Modifiers::OpenAppleShift.into() => {
                Some(Modifiers::OpenAppleShift)
            }
            _ if modifier_scan_code == Modifiers::OpenAppleShiftReset.into() => {
                Some(Modifiers::OpenAppleShiftReset)
            }
            _ if modifier_scan_code == Modifiers::OpenAppleShiftResetControl.into() => {
                Some(Modifiers::OpenAppleShiftResetControl)
            }
            // closed apple
            _ if modifier_scan_code == Modifiers::ClosedAppleControl.into() => {
                Some(Modifiers::ClosedAppleControl)
            }
            _ if modifier_scan_code == Modifiers::ClosedAppleReset.into() => {
                Some(Modifiers::ClosedAppleReset)
            }
            _ if modifier_scan_code == Modifiers::ClosedAppleResetControl.into() => {
                Some(Modifiers::ClosedAppleResetControl)
            }
            _ if modifier_scan_code == Modifiers::ClosedAppleShift.into() => {
                Some(Modifiers::ClosedAppleShift)
            }
            _ if modifier_scan_code == Modifiers::ClosedAppleShiftReset.into() => {
                Some(Modifiers::ClosedAppleShiftReset)
            }
            _ if modifier_scan_code == Modifiers::ClosedAppleShiftResetControl.into() => {
                Some(Modifiers::ClosedAppleShiftResetControl)
            }
            // open + closed apple
            _ if modifier_scan_code == Modifiers::OpenClosedAppleControl.into() => {
                Some(Modifiers::OpenClosedAppleControl)
            }
            _ if modifier_scan_code == Modifiers::OpenClosedAppleReset.into() => {
                Some(Modifiers::OpenClosedAppleReset)
            }
            _ if modifier_scan_code == Modifiers::OpenClosedAppleResetControl.into() => {
                Some(Modifiers::OpenClosedAppleResetControl)
            }
            _ if modifier_scan_code == Modifiers::OpenClosedAppleShift.into() => {
                Some(Modifiers::OpenClosedAppleShift)
            }
            _ if modifier_scan_code == Modifiers::OpenClosedAppleShiftReset.into() => {
                Some(Modifiers::OpenClosedAppleShiftReset)
            }
            _ if modifier_scan_code == Modifiers::OpenClosedAppleShiftResetControl.into() => {
                Some(Modifiers::OpenClosedAppleShiftResetControl)
            }
            // control
            _ if modifier_scan_code == Modifiers::ControlReset.into() => {
                Some(Modifiers::ControlReset)
            }
            _ if modifier_scan_code == Modifiers::ControlResetShift.into() => {
                Some(Modifiers::ControlResetShift)
            }
            _ if modifier_scan_code == Modifiers::ControlShift.into() => {
                Some(Modifiers::ControlShift)
            }
            _ => None,
        }
    }
    pub fn outer_as_string(self) -> String {
        match self {
            Self::Bare => "bare".to_string(),
            Self::OpenApple => "open".to_string(),
            Self::ClosedApple => "closed".to_string(),
            Self::Control => "control".to_string(),
            Self::Reset => "reset".to_string(),
            Self::Shift => "shift".to_string(),
            _ => "".to_string(),
        }
    }
    pub fn is_multiple(self) -> bool {
        match self {
            Self::Bare => false,
            Self::OpenApple => false,
            Self::ClosedApple => false,
            Self::Control => false,
            Self::Reset => false,
            Self::Shift => false,
            _ => true,
        }
    }
}

impl Into<u8> for Modifiers {
    fn into(self) -> u8 {
        match self {
            Self::Bare => 0x0u8,
            Self::OpenApple => 0x40,
            Self::ClosedApple => 0x80,
            Self::OpenClosedApple => 0x40 + 0x80,
            Self::Control => 0x01,
            Self::Reset => 0x02,
            Self::Shift => 0x04,
            // Open Apple Combined
            Self::OpenAppleControl => 0x40 + 0x01, // SW0
            Self::OpenAppleReset => 0x40 + 0x02,   // SW0
            Self::OpenAppleResetControl => 0x40 + 0x02 + 0x01, // SW0
            Self::OpenAppleShift => 0x40 + 0x04,   // SW0
            Self::OpenAppleShiftReset => 0x40 + 0x04 + 0x02, // SW0
            Self::OpenAppleShiftResetControl => 0x40 + 0x04 + 0x02 + 0x01, // SW0

            // Closed Apple Combined
            Self::ClosedAppleControl => 0x80 + 0x01, // SW0
            Self::ClosedAppleReset => 0x80 + 0x02,   // SW0
            Self::ClosedAppleResetControl => 0x80 + 0x02 + 0x01, // SW0
            Self::ClosedAppleShift => 0x80 + 0x04,   // SW0
            Self::ClosedAppleShiftReset => 0x80 + 0x04 + 0x02, // SW0
            Self::ClosedAppleShiftResetControl => 0x80 + 0x04 + 0x02 + 0x01, // SW0

            // Open + Closed Apple Combined
            Self::OpenClosedAppleControl => 0x40 + 0x80 + 0x01, // SW0
            Self::OpenClosedAppleReset => 0x40 + 0x80 + 0x02,   // SW0
            Self::OpenClosedAppleResetControl => 0x40 + 0x80 + 0x02 + 0x01, // SW0
            Self::OpenClosedAppleShift => 0x40 + 0x80 + 0x04,   // SW0
            Self::OpenClosedAppleShiftReset => 0x40 + 0x80 + 0x04 + 0x02, // SW0
            Self::OpenClosedAppleShiftResetControl => 0x40 + 0x80 + 0x04 + 0x02 + 0x01, // SW0

            // Control Combined
            Self::ControlReset => 0x01 + 0x02,
            Self::ControlResetShift => 0x01 + 0x02 + 0x04,
            Self::ControlShift => 0x01 + 0x04,
        }
    }
}

impl From<u8> for Modifiers {
    fn from(value: u8) -> Modifiers {
        /*
        defmt::info!(
            "modifier from {}",
            utils::hex::u8_to_hex_string(value).as_str()
        );
        */
        Modifiers::get(value).unwrap()
    }
}

impl PartialEq for Modifiers {
    fn eq(&self, other: &Self) -> bool {
        let cmp_inner: u8 = other.clone().into();
        let self_inner: u8 = self.clone().into();
        self_inner == cmp_inner
    }
}

#[derive(Clone)]
pub enum KbDriverInput {
    KeyDownEvent((Modifiers, u8, String)),
    KeyUpEvent((Modifiers, u8, String)),
    KeyDown,
    KeyUp,
}

impl KbDriverInput {
    pub fn from_key(modifier_scan_code: u8, key: Key, input: Self) -> Self {
        let modifier_got = Modifiers::get(modifier_scan_code);

        match modifier_got {
            Some(modifier) => match input {
                Self::KeyUp => {
                    Self::KeyUpEvent((modifier, key.action.parse().unwrap(), "".to_string()))
                }
                Self::KeyDown => {
                    Self::KeyDownEvent((modifier, key.action.parse().unwrap(), "".to_string()))
                }
                _ => Self::KeyDownEvent((modifier, key.action.parse().unwrap(), "".to_string())),
            },
            None => match input {
                Self::KeyUp => {
                    Self::KeyUpEvent((Modifiers::Bare, key.action.parse().unwrap(), "".to_string()))
                }
                Self::KeyDown => Self::KeyDownEvent((
                    Modifiers::Bare,
                    key.action.parse().unwrap(),
                    "".to_string(),
                )),
                _ => Self::KeyDownEvent((
                    Modifiers::Bare,
                    key.action.parse().unwrap(),
                    "".to_string(),
                )),
            },
        }
    }
}

pub const A2PI_DESCRIPTOR: &[u8] = &[
    0x05, 0x01, // Usage Page (Generic Desktop Ctrls)
    0x09, 0x06, // Usage (Keyboard)
    0xA1, 0x01, // Collection (Application)
    // Modifier Keys
    0x05, 0x07, //   Usage Page (Kbrd/Keypad)
    0x19, 0xE0, //   Usage Minimum (0xE0)
    0x29, 0xE7, //   Usage Maximum (0xE7)
    0x15, 0x00, //   Logical Minimum (0)
    0x25, 0x01, //   Logical Maximum (1)
    0x95, 0x08, //   Report Count (8)
    0x75, 0x01, //   Report Size (1)
    0x81, 0x02, //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    // Reserved Byte
    0x95, 0x01, //   Report Count (1)
    0x75, 0x08, //   Report Size (8)
    0x81, 0x01, //   Input (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    // LEDs
    0x05, 0x08, //   Usage Page (LEDs)
    0x19, 0x01, //   Usage Minimum (Num Lock)
    0x29, 0x05, //   Usage Maximum (Kana)
    0x95, 0x05, //   Report Count (5)
    0x75, 0x01, //   Report Size (1)
    0x91,
    0x02, //   Output (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    // LED Padding
    0x95, 0x01, //   Report Count (1)
    0x75, 0x03, //   Report Size (3)
    0x91,
    0x01, //   Output (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    // Keycodes
    0x05, 0x07, //   Usage Page (Kbrd/Keypad)
    0x19, 0x00, //   Usage Minimum (0x00)
    0x29, 0xDD, //   Usage Maximum (0xDD) - TODO - double check this
    0x15, 0x00, //   Logical Minimum (0)
    0x26, 0xFF,
    0x00, //   Logical Maximum (255) - TOOD - double check max and trailing 0x00 byte
    0x95, 0x06, //   Report Count (6)
    0x75, 0x08, //   Report Size (8)
    0x81, 0x00, //   Input (Data,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0xC0, // End Collection
];
