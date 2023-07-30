use alloc::vec;
use usbd_human_interface_device::page::Keyboard;

use super::LayoutWithHID;

pub fn hoist_hid_keyboard_map() -> LayoutWithHID {
    vec![
        (
            "0x01",
            [("0x00", (0x00, 0x00, vec![Keyboard::LeftControl]))].to_vec(),
        ),
        (
            "0x02", // reset (inaccessble)
            [].to_vec(),
        ),
        (
            "0x03", // control + reset
            [("0x00", (0x00, 0x00, vec![Keyboard::Tab]))].to_vec(),
        ),
        (
            "0x04", // shift
            [("0x00", (0x00, 0x00, vec![Keyboard::LeftShift]))].to_vec(),
        ),
        (
            "0x07", // control (0x01) + reset (0x02) + shift (0x04)
            [(
                "0x00",
                (0x00, 0x00, vec![Keyboard::LeftShift, Keyboard::Tab]),
            )]
            .to_vec(),
        ),
        (
            "0x40",
            [
                (
                    "0x00",
                    (0x00, 0x00, vec![Keyboard::LeftAlt]), // / :: KEY_SLASH
                ),
                (
                    "0x2f",
                    (
                        0xaf,
                        0x2f,
                        vec![Keyboard::LeftAlt, Keyboard::LeftShift, Keyboard::Keyboard7],
                    ), // / :: KEY_SLASH
                ),
                (
                    "0x35",
                    (0xb5, 0x35, vec![Keyboard::LeftAlt, Keyboard::Keyboard5]), // 5 :: KEY_5
                ),
                (
                    "0x36",
                    (0xb6, 0x36, vec![Keyboard::LeftAlt, Keyboard::Keyboard6]), // 6 :: KEY_6
                ),
                (
                    "0x37",
                    (0xb7, 0x37, vec![Keyboard::LeftAlt, Keyboard::Keyboard7]), // 7 :: KEY_7
                ),
                (
                    "0x38",
                    (0xb8, 0x38, vec![Keyboard::LeftAlt, Keyboard::Keyboard8]), // 8 :: KEY_8
                ),
                (
                    "0x39",
                    (0xb9, 0x39, vec![Keyboard::LeftAlt, Keyboard::Keyboard9]), // 9 :: KEY_9
                ),
                (
                    "0x74",
                    (0xf4, 0x74, vec![Keyboard::LeftGUI, Keyboard::T]), // t :: KEY_T
                ),
                (
                    "0x77",
                    (0xf7, 0x77, vec![Keyboard::LeftGUI, Keyboard::W]), // w :: KEY_W
                ),
                (
                    "0x76",
                    (0xf6, 0x76, vec![Keyboard::LeftGUI, Keyboard::V]), // v :: KEY_V
                ),
                (
                    "0x61",
                    (0xe1, 0x61, vec![Keyboard::LeftGUI, Keyboard::A]), // a :: KEY_A
                ),
                (
                    "0x63",
                    (0xe3, 0x63, vec![Keyboard::LeftGUI, Keyboard::C]), // c :: KEY_C
                ),
                (
                    "0x6e",
                    (0xee, 0x6e, vec![Keyboard::RightAlt, Keyboard::N]), // n :: KEY_N
                ),
                (
                    "0x6c",
                    (0xec, 0x6c, vec![Keyboard::RightAlt, Keyboard::L]), // l :: KEY_L
                ),
                (
                    "0x0d",
                    (0x8d, 0x0d, vec![Keyboard::Tab]), // Enter :: KEY_ENTER
                ),
            ]
            .to_vec(),
        ),
        (
            "0x44", // shift
            [(
                "0x00",
                (0x00, 0x00, vec![Keyboard::LeftAlt, Keyboard::LeftShift]),
            )]
            .to_vec(),
        ),
        (
            "0x80",
            [
                (
                    "0x00",
                    (0x00, 0x00, vec![Keyboard::LeftGUI]), // / :: KEY_SLASH
                ),
                (
                    "0x6e",
                    (0xee, 0x6e, vec![Keyboard::LeftGUI, Keyboard::N]), // n :: KEY_N
                ),
                (
                    "0x23",
                    (0x23, 0x23, vec![Keyboard::LeftArrow]), // H
                ),
                (
                    "0x26",
                    (0x26, 0x26, vec![Keyboard::DownArrow]), // J
                ),
                (
                    "0x27",
                    (0x27, 0x27, vec![Keyboard::UpArrow]), // K
                ),
                (
                    "0x29",
                    (0x29, 0x29, vec![Keyboard::RightArrow]), // L
                ),
                (
                    "0x0d",
                    (0x8d, 0x0d, vec![Keyboard::LeftShift, Keyboard::Tab]), // Enter :: KEY_ENTER
                ),
                (
                    "0x37",
                    (0xb7, 0x37, vec![Keyboard::F7]), // 7 :: KEY_7
                ),
                (
                    "0x38",
                    (0xb8, 0x38, vec![Keyboard::F8]), // 8 :: KEY_8
                ),
                (
                    "0x39",
                    (0xb9, 0x39, vec![Keyboard::F9]), // 9 :: KEY_9
                ),
                (
                    "0x30",
                    (0xb0, 0x30, vec![Keyboard::F10]), // 0 :: KEY_0
                ),
                (
                    "0x7e",
                    (0xfe, 0x7e, vec![Keyboard::F11]), // ~ :: SHIFT+KEY_GRAVE
                ),
                (
                    "0x27",
                    (0xa7, 0x27, vec![Keyboard::F12]), // ' :: KEY_APOSTROPHE
                ),
            ]
            .to_vec(),
        ),
        (
            "0xC0",
            [
                (
                    "0x00",
                    (0x00, 0x00, vec![Keyboard::NoEventIndicated]), // n :: KEY_N
                ),
                (
                    "0x6e",
                    (0xee, 0x6e, vec![Keyboard::LeftGUI, Keyboard::N]), // n :: KEY_N
                ),
                (
                    "0x6c",
                    (0xec, 0x6c, vec![Keyboard::LeftGUI, Keyboard::L]), // l :: KEY_L
                ),
                (
                    "0x0d",
                    (0x8d, 0x0d, vec![Keyboard::LeftGUI, Keyboard::Tab]), // Enter :: KEY_ENTER
                ),
            ]
            .to_vec(),
        ),
        (
            "0x00",
            [
                // first (top) row
                (
                    "0x00",
                    (0x00, 0x00, vec![Keyboard::NonUSBackslash]), // Escape
                ),
                (
                    "0x01",
                    (0x01, 0x01, vec![Keyboard::Keyboard1]), // 1
                ),
                (
                    "0x02",
                    (0x02, 0x02, vec![Keyboard::Keyboard2]), // 1
                ),
                (
                    "0x03",
                    (0x03, 0x03, vec![Keyboard::Keyboard3]), // 3
                ),
                (
                    "0x04",
                    (0x04, 0x04, vec![Keyboard::Keyboard4]), // 4
                ),
                (
                    "0x06",
                    (0x06, 0x06, vec![Keyboard::Keyboard5]), // 5
                ),
                (
                    "0x05",
                    (0x05, 0x05, vec![Keyboard::Keyboard6]), // 6
                ),
                (
                    "0x07",
                    (0x07, 0x07, vec![Keyboard::Keyboard7]), // 7
                ),
                (
                    "0x08",
                    (0x08, 0x08, vec![Keyboard::Keyboard8]), // 8
                ),
                (
                    "0x09",
                    (0x09, 0x09, vec![Keyboard::Keyboard9]), // 9
                ),
                (
                    "0x48",
                    (0x48, 0x48, vec![Keyboard::Keyboard0]), // 0
                ),
                (
                    "0x49",
                    (0x49, 0x49, vec![Keyboard::Minus]), // ß/?
                ),
                (
                    "0x47",
                    (0x47, 0x47, vec![Keyboard::Equal]), // ´/`
                ),
                (
                    "0x76",
                    (0x76, 0x76, vec![Keyboard::DeleteBackspace]), // ´/`
                ),
                // second row
                (
                    "0x10",
                    (0x10, 0x10, vec![Keyboard::Escape]), // Tab
                ),
                (
                    "0x11",
                    (0x11, 0x11, vec![Keyboard::Q]), // Q
                ),
                (
                    "0x12",
                    (0x12, 0x12, vec![Keyboard::W]), // W
                ),
                (
                    "0x13",
                    (0x13, 0x13, vec![Keyboard::E]), // E
                ),
                (
                    "0x14",
                    (0x14, 0x14, vec![Keyboard::R]), // R
                ),
                (
                    "0x16",
                    (0x16, 0x16, vec![Keyboard::T]), // T
                ),
                (
                    "0x15",
                    (0x15, 0x15, vec![Keyboard::Y]), // Y (ansi) :: Z (german)
                ),
                (
                    "0x17",
                    (0x17, 0x17, vec![Keyboard::U]), // U
                ),
                (
                    "0x18",
                    (0x18, 0x18, vec![Keyboard::I]), // I
                ),
                (
                    "0x19",
                    (0x19, 0x19, vec![Keyboard::O]), // O
                ),
                (
                    "0x57",
                    (0x57, 0x57, vec![Keyboard::P]), // P
                ),
                (
                    "0x58",
                    (0x58, 0x58, vec![Keyboard::NoEventIndicated]), // u umlaut
                ),
                (
                    "0x59",
                    (0x59, 0x59, vec![Keyboard::RightBrace]), // +/*
                ),
                // third row
                (
                    "0x20",
                    (0x20, 0x20, vec![Keyboard::A]), // A
                ),
                (
                    "0x22",
                    (0x22, 0x22, vec![Keyboard::S]), // S
                ),
                (
                    "0x21",
                    (0x21, 0x21, vec![Keyboard::D]), // D
                ),
                (
                    "0x24",
                    (0x24, 0x24, vec![Keyboard::F]), // F
                ),
                (
                    "0x25",
                    (0x25, 0x25, vec![Keyboard::G]), // G
                ),
                (
                    "0x23",
                    (0x23, 0x23, vec![Keyboard::H]), // H
                ),
                (
                    "0x26",
                    (0x26, 0x26, vec![Keyboard::J]), // J
                ),
                (
                    "0x27",
                    (0x27, 0x27, vec![Keyboard::K]), // K
                ),
                (
                    "0x29",
                    (0x29, 0x29, vec![Keyboard::L]), // L
                ),
                (
                    "0x28",
                    (0x28, 0x28, vec![Keyboard::NoEventIndicated]), // o umlaut
                ),
                (
                    "0x69",
                    (0x69, 0x69, vec![Keyboard::NoEventIndicated]), // a umlaut
                ),
                (
                    "0x46",
                    (0x46, 0x46, vec![Keyboard::Backslash]), // #/'
                ),
                // bottom row
                (
                    "0x56",
                    (0x56, 0x56, vec![Keyboard::Grave]), // </>
                ),
                (
                    "0x30",
                    (0x30, 0x30, vec![Keyboard::Z]), // Z (ansi) or Y (german)
                ),
                (
                    "0x31",
                    (0x31, 0x31, vec![Keyboard::X]), // X
                ),
                (
                    "0x32",
                    (0x32, 0x32, vec![Keyboard::C]), // C
                ),
                (
                    "0x33",
                    (0x33, 0x33, vec![Keyboard::V]), // V
                ),
                (
                    "0x34",
                    (0x34, 0x34, vec![Keyboard::B]), // B
                ),
                (
                    "0x35",
                    (0x35, 0x35, vec![Keyboard::N]), // N
                ),
                (
                    "0x36",
                    (0x36, 0x36, vec![Keyboard::M]), // M
                ),
                (
                    "0x37",
                    (0x37, 0x37, vec![Keyboard::Comma]), // ,/;
                ),
                (
                    "0x38",
                    (0x38, 0x38, vec![Keyboard::Dot]), // ./:
                ),
                (
                    "0x39",
                    (0x39, 0x39, vec![Keyboard::ForwardSlash]), // -/_
                ),
                // Enter Key
                (
                    "0x66",
                    (0x66, 0x66, vec![Keyboard::ReturnEnter]), // -/_
                ),
                (
                    "0x68",
                    (0x68, 0x68, vec![Keyboard::Space]), // -/_
                ),
            ]
            .to_vec(),
        ),
    ]
}
