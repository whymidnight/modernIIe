// #[cfg(feature = "no_std")]
use alloc::format;
// #[cfg(feature = "no_std")]
use alloc::string::String;
use hex_display::HexDisplayExt;

// #[rage(due_to = "no_std")]
pub fn u8_to_hex_string(number: u8) -> String {
    format!("0x{}", &[number].hex())
}

pub fn decode_hex(s: &str) -> u8 {
    u8::from_str_radix(s.trim_start_matches("0x"), 16).unwrap()
}
