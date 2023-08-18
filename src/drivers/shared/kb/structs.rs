#[cfg(feature = "no-std")]
use alloc::vec::Vec;

#[cfg(feature = "no-std")]
use crate::drivers::no_std::kb::kbmap::KeyboardMapEntrant;
#[cfg(feature = "no-std")]
use alloc::format;
#[cfg(feature = "no-std")]
use alloc::string::String;

#[cfg_attr(feature = "std", derive(Deserialize))]
#[derive(Clone)]
pub struct Key {
    pub key: String,    // key down scan code
    pub action: String, // key up scan code
    #[cfg(feature = "no-std")]
    pub usb_hid: Vec<KeyboardMapEntrant>,
}

#[cfg(feature = "no-std")]
impl Key {
    pub fn define(key: u8, action: u8, usb_hid: Vec<KeyboardMapEntrant>) -> Self {
        Self {
            key: format!("{key}"),
            action: format!("{action}"),
            usb_hid,
        }
    }
    pub fn to_ascii(self) -> String {
        self.action.to_ascii_lowercase()
    }
}
