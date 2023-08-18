use alloc::vec;
use usbd_human_interface_device::page::{Consumer, Keyboard};

use super::LayoutWithHID;

#[derive(Clone)]
pub enum KeyboardMapEntrant {
    Keyboard(Keyboard),
    Consumer(Consumer),
}

impl PartialEq for KeyboardMapEntrant {
    fn eq(&self, other: &Self) -> bool {
        match self {
            KeyboardMapEntrant::Keyboard(keyboard) => match other {
                KeyboardMapEntrant::Keyboard(other_keyboard) => {
                    let self_u8: u8 = keyboard.clone().into();
                    self_u8 == other_keyboard.clone().into()
                }
                KeyboardMapEntrant::Consumer(_) => false,
            },
            KeyboardMapEntrant::Consumer(consumer) => match other {
                KeyboardMapEntrant::Keyboard(_) => false,
                KeyboardMapEntrant::Consumer(other_consumer) => {
                    let self_u16: u16 = consumer.clone().into();
                    self_u16 == other_consumer.clone().into()
                }
            },
        }
    }
}

impl Into<u8> for KeyboardMapEntrant {
    fn into(self) -> u8 {
        match self {
            KeyboardMapEntrant::Keyboard(keyboard) => {
                let keyboard_u8: u8 = keyboard.into();
                keyboard_u8
            }
            KeyboardMapEntrant::Consumer(consumer) => {
                let consumer_u16: u16 = consumer.into();
                consumer_u16 as u8
            }
        }
    }
}

impl Into<u16> for KeyboardMapEntrant {
    fn into(self) -> u16 {
        match self {
            KeyboardMapEntrant::Keyboard(keyboard) => 0,
            KeyboardMapEntrant::Consumer(consumer) => consumer.into(),
        }
    }
}

pub fn hoist_hid_keyboard_map() -> LayoutWithHID {
    vec![
        (
            "0x01",
            [(
                "0x00",
                (
                    0x00,
                    0x00,
                    vec![KeyboardMapEntrant::Keyboard(Keyboard::LeftControl)],
                ),
            )]
            .to_vec(),
        ),
        (
            "0x02", // reset (inaccessble)
            [].to_vec(),
        ),
        (
            "0x03", // control + reset
            [(
                "0x00",
                (
                    0x00,
                    0x00,
                    vec![KeyboardMapEntrant::Keyboard(Keyboard::Tab)],
                ),
            )]
            .to_vec(),
        ),
        (
            "0x04", // shift
            [(
                "0x00",
                (
                    0x00,
                    0x00,
                    vec![KeyboardMapEntrant::Keyboard(Keyboard::LeftShift)],
                ),
            )]
            .to_vec(),
        ),
        (
            "0x05", // control (0x01) + shift (0x04)
            [(
                "0x00",
                (
                    0x00,
                    0x00,
                    vec![
                        KeyboardMapEntrant::Keyboard(Keyboard::LeftControl),
                        KeyboardMapEntrant::Keyboard(Keyboard::LeftShift),
                    ],
                ),
            )]
            .to_vec(),
        ),
        (
            "0x07", // control (0x01) + reset (0x02) + shift (0x04)
            [(
                "0x00",
                (
                    0x00,
                    0x00,
                    vec![
                        KeyboardMapEntrant::Keyboard(Keyboard::LeftControl),
                        KeyboardMapEntrant::Keyboard(Keyboard::LeftShift),
                        KeyboardMapEntrant::Keyboard(Keyboard::Tab),
                    ],
                ),
            )]
            .to_vec(),
        ),
        (
            "0x40",
            [
                (
                    "0x00",
                    (
                        0x00,
                        0x00,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::LeftAlt)],
                    ), // / :: KEY_SLASH
                ),
                (
                    "0x20",
                    (
                        0x20,
                        0x20,
                        vec![
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftGUI),
                            KeyboardMapEntrant::Keyboard(Keyboard::A),
                        ],
                    ), // A
                ),
                (
                    "0x32",
                    (
                        0x32,
                        0x32,
                        vec![
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftGUI),
                            KeyboardMapEntrant::Keyboard(Keyboard::C),
                        ],
                    ), // C
                ),
                (
                    "0x33",
                    (
                        0x33,
                        0x33,
                        vec![
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftGUI),
                            KeyboardMapEntrant::Keyboard(Keyboard::V),
                        ],
                    ), // V
                ),
                (
                    "0x12",
                    (
                        0x12,
                        0x12,
                        vec![
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftGUI),
                            KeyboardMapEntrant::Keyboard(Keyboard::W),
                        ],
                    ), // W
                ),
                (
                    "0x14",
                    (
                        0x14,
                        0x14,
                        vec![
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftGUI),
                            KeyboardMapEntrant::Keyboard(Keyboard::R),
                        ],
                    ), // R
                ),
                (
                    "0x16",
                    (
                        0x16,
                        0x16,
                        vec![
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftGUI),
                            KeyboardMapEntrant::Keyboard(Keyboard::T),
                        ],
                    ), // T
                ),
                (
                    "0x31",
                    (
                        0x31,
                        0x31,
                        vec![
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftGUI),
                            KeyboardMapEntrant::Keyboard(Keyboard::X),
                        ],
                    ), // X
                ),
                (
                    "0x29",
                    (
                        0x29,
                        0x29,
                        vec![
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftGUI),
                            KeyboardMapEntrant::Keyboard(Keyboard::L),
                        ],
                    ), // L
                ),
                (
                    "0x35",
                    (
                        0x35,
                        0x35,
                        vec![
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftGUI),
                            KeyboardMapEntrant::Keyboard(Keyboard::N),
                        ],
                    ), // N
                ),
            ]
            .to_vec(),
        ),
        (
            "0x44", // shift
            [
                (
                    "0x00",
                    (
                        0x00,
                        0x00,
                        vec![
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftAlt),
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftShift),
                        ],
                    ),
                ),
                (
                    "0x29",
                    (
                        0x29,
                        0x29,
                        vec![
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftAlt),
                            KeyboardMapEntrant::Keyboard(Keyboard::L),
                        ],
                    ), // L
                ),
                (
                    "0x35",
                    (
                        0x35,
                        0x35,
                        vec![
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftAlt),
                            KeyboardMapEntrant::Keyboard(Keyboard::N),
                        ],
                    ), // N
                ),
                (
                    "0x01",
                    (
                        0x01,
                        0x01,
                        vec![
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftGUI),
                            KeyboardMapEntrant::Keyboard(Keyboard::Keyboard1),
                        ],
                    ), // 1
                ),
                (
                    "0x02",
                    (
                        0x02,
                        0x02,
                        vec![
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftGUI),
                            KeyboardMapEntrant::Keyboard(Keyboard::Keyboard2),
                        ],
                    ), // 2
                ),
                (
                    "0x03",
                    (
                        0x03,
                        0x03,
                        vec![
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftGUI),
                            KeyboardMapEntrant::Keyboard(Keyboard::Keyboard3),
                        ],
                    ), // 3
                ),
                (
                    "0x04",
                    (
                        0x04,
                        0x04,
                        vec![
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftGUI),
                            KeyboardMapEntrant::Keyboard(Keyboard::Keyboard4),
                        ],
                    ), // 4
                ),
                (
                    "0x06",
                    (
                        0x06,
                        0x06,
                        vec![
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftGUI),
                            KeyboardMapEntrant::Keyboard(Keyboard::Keyboard5),
                        ],
                    ), // 5
                ),
                (
                    "0x05",
                    (
                        0x05,
                        0x05,
                        vec![
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftGUI),
                            KeyboardMapEntrant::Keyboard(Keyboard::Keyboard6),
                        ],
                    ), // 6
                ),
                (
                    "0x07",
                    (
                        0x07,
                        0x07,
                        vec![
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftAlt),
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftShift),
                            KeyboardMapEntrant::Keyboard(Keyboard::Keyboard7),
                        ],
                    ), // 7
                ),
                (
                    "0x08",
                    (
                        0x08,
                        0x08,
                        vec![
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftGUI),
                            KeyboardMapEntrant::Keyboard(Keyboard::Keyboard8),
                        ],
                    ), // 8
                ),
                (
                    "0x09",
                    (
                        0x09,
                        0x09,
                        vec![
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftGUI),
                            KeyboardMapEntrant::Keyboard(Keyboard::Keyboard9),
                        ],
                    ), // 9
                ),
                (
                    "0x48",
                    (
                        0x48,
                        0x48,
                        vec![
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftGUI),
                            KeyboardMapEntrant::Keyboard(Keyboard::Keyboard0),
                        ],
                    ), // 0
                ),
            ]
            .to_vec(),
        ),
        (
            "0x80",
            [
                (
                    "0x00",
                    (
                        0x00,
                        0x00,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::LeftGUI)],
                    ), // / :: KEY_SLASH
                ),
                (
                    "0x6e",
                    (
                        0xee,
                        0x6e,
                        vec![
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftGUI),
                            KeyboardMapEntrant::Keyboard(Keyboard::N),
                        ],
                    ), // n :: KEY_N
                ),
                (
                    "0x23",
                    (
                        0x23,
                        0x23,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::LeftArrow)],
                    ), // H
                ),
                (
                    "0x26",
                    (
                        0x26,
                        0x26,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::DownArrow)],
                    ), // J
                ),
                (
                    "0x27",
                    (
                        0x27,
                        0x27,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::UpArrow)],
                    ), // K
                ),
                (
                    "0x29",
                    (
                        0x29,
                        0x29,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::RightArrow)],
                    ), // L
                ),
                (
                    "0x01",
                    (0x01, 0x01, vec![KeyboardMapEntrant::Keyboard(Keyboard::F1)]), // 1
                ),
                (
                    "0x02",
                    (0x02, 0x02, vec![KeyboardMapEntrant::Keyboard(Keyboard::F2)]), // 2
                ),
                (
                    "0x03",
                    (0x03, 0x03, vec![KeyboardMapEntrant::Keyboard(Keyboard::F3)]), // 3
                ),
                (
                    "0x04",
                    (0x04, 0x04, vec![KeyboardMapEntrant::Keyboard(Keyboard::F4)]), // 4
                ),
                (
                    "0x06",
                    (0x06, 0x06, vec![KeyboardMapEntrant::Keyboard(Keyboard::F5)]), // 5
                ),
                (
                    "0x05",
                    (0x05, 0x05, vec![KeyboardMapEntrant::Keyboard(Keyboard::F6)]), // 6
                ),
                (
                    "0x07",
                    (
                        0x07,
                        0x07,
                        vec![KeyboardMapEntrant::Consumer(Consumer::Rewind)],
                    ), // 7
                ),
                (
                    "0x08",
                    (
                        0x08,
                        0x08,
                        vec![KeyboardMapEntrant::Consumer(Consumer::PlayPause)],
                    ), // 8
                ),
                (
                    "0x09",
                    (
                        0x09,
                        0x09,
                        vec![KeyboardMapEntrant::Consumer(Consumer::FastForward)],
                    ), // 9
                ),
                (
                    "0x48",
                    (
                        0x48,
                        0x48,
                        vec![KeyboardMapEntrant::Consumer(Consumer::Mute)],
                    ), // 0
                ),
                (
                    "0x49",
                    (
                        0x49,
                        0x49,
                        vec![KeyboardMapEntrant::Consumer(Consumer::VolumeDecrement)],
                    ), // ß/?
                ),
                (
                    "0x47",
                    (
                        0x47,
                        0x47,
                        vec![KeyboardMapEntrant::Consumer(Consumer::VolumeIncrement)],
                    ), // ´/`
                ),
            ]
            .to_vec(),
        ),
        (
            "0x85",
            [(
                "0x00",
                (
                    0x00,
                    0x00,
                    vec![KeyboardMapEntrant::Keyboard(Keyboard::NoEventIndicated)],
                ), // n :: KEY_N
            )]
            .to_vec(),
        ),
        (
            "0xC0",
            [
                (
                    "0x00",
                    (
                        0x00,
                        0x00,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::NoEventIndicated)],
                    ), // n :: KEY_N
                ),
                (
                    "0x23",
                    (
                        0x23,
                        0x23,
                        vec![
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftControl),
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftArrow),
                        ],
                    ), // H
                ),
                (
                    "0x26",
                    (
                        0x26,
                        0x26,
                        vec![
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftControl),
                            KeyboardMapEntrant::Keyboard(Keyboard::DownArrow),
                        ],
                    ), // J
                ),
                (
                    "0x27",
                    (
                        0x27,
                        0x27,
                        vec![
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftControl),
                            KeyboardMapEntrant::Keyboard(Keyboard::UpArrow),
                        ],
                    ), // K
                ),
                (
                    "0x29",
                    (
                        0x29,
                        0x29,
                        vec![
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftControl),
                            KeyboardMapEntrant::Keyboard(Keyboard::RightArrow),
                        ],
                    ), // L
                ),
                (
                    "0x18",
                    (
                        0x18,
                        0x18,
                        vec![
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftAlt),
                            KeyboardMapEntrant::Keyboard(Keyboard::LeftGUI),
                            KeyboardMapEntrant::Keyboard(Keyboard::I),
                        ],
                    ), // I
                ),
            ]
            .to_vec(),
        ),
        (
            "0xC4",
            [(
                "0x00",
                (
                    0x00,
                    0x00,
                    vec![KeyboardMapEntrant::Keyboard(Keyboard::NoEventIndicated)],
                ), // n :: KEY_N
            )]
            .to_vec(),
        ),
        (
            "0x00",
            [
                // first (top) row
                (
                    "0x00",
                    (
                        0x00,
                        0x00,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::NonUSBackslash)],
                    ), // Escape
                ),
                (
                    "0x01",
                    (
                        0x01,
                        0x01,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::Keyboard1)],
                    ), // 1
                ),
                (
                    "0x02",
                    (
                        0x02,
                        0x02,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::Keyboard2)],
                    ), // 2
                ),
                (
                    "0x03",
                    (
                        0x03,
                        0x03,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::Keyboard3)],
                    ), // 3
                ),
                (
                    "0x04",
                    (
                        0x04,
                        0x04,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::Keyboard4)],
                    ), // 4
                ),
                (
                    "0x06",
                    (
                        0x06,
                        0x06,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::Keyboard5)],
                    ), // 5
                ),
                (
                    "0x05",
                    (
                        0x05,
                        0x05,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::Keyboard6)],
                    ), // 6
                ),
                (
                    "0x07",
                    (
                        0x07,
                        0x07,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::Keyboard7)],
                    ), // 7
                ),
                (
                    "0x08",
                    (
                        0x08,
                        0x08,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::Keyboard8)],
                    ), // 8
                ),
                (
                    "0x09",
                    (
                        0x09,
                        0x09,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::Keyboard9)],
                    ), // 9
                ),
                (
                    "0x48",
                    (
                        0x48,
                        0x48,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::Keyboard0)],
                    ), // 0
                ),
                (
                    "0x49",
                    (
                        0x49,
                        0x49,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::Minus)],
                    ), // ß/?
                ),
                (
                    "0x47",
                    (
                        0x47,
                        0x47,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::Equal)],
                    ), // ´/`
                ),
                (
                    "0x76",
                    (
                        0x76,
                        0x76,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::DeleteBackspace)],
                    ), // ´/`
                ),
                // second row
                (
                    "0x10",
                    (
                        0x10,
                        0x10,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::Escape)],
                    ), // Tab
                ),
                (
                    "0x11",
                    (0x11, 0x11, vec![KeyboardMapEntrant::Keyboard(Keyboard::Q)]), // Q
                ),
                (
                    "0x12",
                    (0x12, 0x12, vec![KeyboardMapEntrant::Keyboard(Keyboard::W)]), // W
                ),
                (
                    "0x13",
                    (0x13, 0x13, vec![KeyboardMapEntrant::Keyboard(Keyboard::E)]), // E
                ),
                (
                    "0x14",
                    (0x14, 0x14, vec![KeyboardMapEntrant::Keyboard(Keyboard::R)]), // R
                ),
                (
                    "0x16",
                    (0x16, 0x16, vec![KeyboardMapEntrant::Keyboard(Keyboard::T)]), // T
                ),
                (
                    "0x15",
                    (0x15, 0x15, vec![KeyboardMapEntrant::Keyboard(Keyboard::Y)]), // Y (ansi) :: Z (german)
                ),
                (
                    "0x17",
                    (0x17, 0x17, vec![KeyboardMapEntrant::Keyboard(Keyboard::U)]), // U
                ),
                (
                    "0x18",
                    (0x18, 0x18, vec![KeyboardMapEntrant::Keyboard(Keyboard::I)]), // I
                ),
                (
                    "0x19",
                    (0x19, 0x19, vec![KeyboardMapEntrant::Keyboard(Keyboard::O)]), // O
                ),
                (
                    "0x57",
                    (0x57, 0x57, vec![KeyboardMapEntrant::Keyboard(Keyboard::P)]), // P
                ),
                (
                    "0x58",
                    (
                        0x58,
                        0x58,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::Tab)],
                    ), // u umlaut
                ),
                (
                    "0x59",
                    (
                        0x59,
                        0x59,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::RightBrace)],
                    ), // +/*
                ),
                // third row
                (
                    "0x20",
                    (0x20, 0x20, vec![KeyboardMapEntrant::Keyboard(Keyboard::A)]), // A
                ),
                (
                    "0x22",
                    (0x22, 0x22, vec![KeyboardMapEntrant::Keyboard(Keyboard::S)]), // S
                ),
                (
                    "0x21",
                    (0x21, 0x21, vec![KeyboardMapEntrant::Keyboard(Keyboard::D)]), // D
                ),
                (
                    "0x24",
                    (0x24, 0x24, vec![KeyboardMapEntrant::Keyboard(Keyboard::F)]), // F
                ),
                (
                    "0x25",
                    (0x25, 0x25, vec![KeyboardMapEntrant::Keyboard(Keyboard::G)]), // G
                ),
                (
                    "0x23",
                    (0x23, 0x23, vec![KeyboardMapEntrant::Keyboard(Keyboard::H)]), // H
                ),
                (
                    "0x26",
                    (0x26, 0x26, vec![KeyboardMapEntrant::Keyboard(Keyboard::J)]), // J
                ),
                (
                    "0x27",
                    (0x27, 0x27, vec![KeyboardMapEntrant::Keyboard(Keyboard::K)]), // K
                ),
                (
                    "0x29",
                    (0x29, 0x29, vec![KeyboardMapEntrant::Keyboard(Keyboard::L)]), // L
                ),
                (
                    "0x28",
                    (
                        0x28,
                        0x28,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::NoEventIndicated)],
                    ), // o umlaut
                ),
                (
                    "0x69",
                    (
                        0x69,
                        0x69,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::NoEventIndicated)],
                    ), // a umlaut
                ),
                (
                    "0x46",
                    (
                        0x46,
                        0x46,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::Backslash)],
                    ), // #/'
                ),
                // bottom row
                (
                    "0x56",
                    (
                        0x56,
                        0x56,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::Grave)],
                    ), // </>
                ),
                (
                    "0x30",
                    (0x30, 0x30, vec![KeyboardMapEntrant::Keyboard(Keyboard::Z)]), // Z (ansi) or Y (german)
                ),
                (
                    "0x31",
                    (0x31, 0x31, vec![KeyboardMapEntrant::Keyboard(Keyboard::X)]), // X
                ),
                (
                    "0x32",
                    (0x32, 0x32, vec![KeyboardMapEntrant::Keyboard(Keyboard::C)]), // C
                ),
                (
                    "0x33",
                    (0x33, 0x33, vec![KeyboardMapEntrant::Keyboard(Keyboard::V)]), // V
                ),
                (
                    "0x34",
                    (0x34, 0x34, vec![KeyboardMapEntrant::Keyboard(Keyboard::B)]), // B
                ),
                (
                    "0x35",
                    (0x35, 0x35, vec![KeyboardMapEntrant::Keyboard(Keyboard::N)]), // N
                ),
                (
                    "0x36",
                    (0x36, 0x36, vec![KeyboardMapEntrant::Keyboard(Keyboard::M)]), // M
                ),
                (
                    "0x37",
                    (
                        0x37,
                        0x37,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::Comma)],
                    ), // ,/;
                ),
                (
                    "0x38",
                    (
                        0x38,
                        0x38,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::Dot)],
                    ), // ./:
                ),
                (
                    "0x39",
                    (
                        0x39,
                        0x39,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::ForwardSlash)],
                    ), // -/_
                ),
                // Enter Key
                (
                    "0x66",
                    (
                        0x66,
                        0x66,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::ReturnEnter)],
                    ), // -/_
                ),
                (
                    "0x68",
                    (
                        0x68,
                        0x68,
                        vec![KeyboardMapEntrant::Keyboard(Keyboard::Space)],
                    ), // -/_
                ),
            ]
            .to_vec(),
        ),
    ]
}
