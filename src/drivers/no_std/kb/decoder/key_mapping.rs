use super::{key_codes::KeyCode, NUM_COLS, NUM_ROWS};

#[rustfmt::skip]
pub const NORMAL_LAYER_MAPPING: [[KeyCode; NUM_ROWS]; NUM_COLS] = [
    [KeyCode::Escape, KeyCode::Tilde, KeyCode::Tab, KeyCode::CapsLock, KeyCode::LeftShift, KeyCode::Fn, KeyCode::Tab, KeyCode::CapsLock, KeyCode::LeftShift, KeyCode::Fn],
    [KeyCode::F1, KeyCode::Num1, KeyCode::Q, KeyCode::A, KeyCode::Empty, KeyCode::LeftCtrl, KeyCode::Tab, KeyCode::CapsLock, KeyCode::LeftShift, KeyCode::Fn],
    [KeyCode::F2, KeyCode::Num2, KeyCode::W, KeyCode::S, KeyCode::Z, KeyCode::LeftAlt, KeyCode::Tab, KeyCode::CapsLock, KeyCode::LeftShift, KeyCode::Fn],
    [KeyCode::F3, KeyCode::Num3, KeyCode::E, KeyCode::D, KeyCode::X, KeyCode::LeftCmd, KeyCode::Tab, KeyCode::CapsLock, KeyCode::LeftShift, KeyCode::Fn],
    [KeyCode::F4, KeyCode::Num4, KeyCode::R, KeyCode::F, KeyCode::C, KeyCode::Empty, KeyCode::Tab, KeyCode::CapsLock, KeyCode::LeftShift, KeyCode::Fn],
    [KeyCode::F5, KeyCode::Num5, KeyCode::T, KeyCode::G, KeyCode::V, KeyCode::Empty, KeyCode::Tab, KeyCode::CapsLock, KeyCode::LeftShift, KeyCode::Fn],
    [KeyCode::Empty, KeyCode::Num6, KeyCode::Y, KeyCode::H, KeyCode::B, KeyCode::Space, KeyCode::Tab, KeyCode::CapsLock, KeyCode::LeftShift, KeyCode::Fn],
    [KeyCode::F6, KeyCode::Num7, KeyCode::U, KeyCode::J, KeyCode::N, KeyCode::Empty, KeyCode::Tab, KeyCode::CapsLock, KeyCode::LeftShift, KeyCode::Fn],
];

#[rustfmt::skip]
pub const FN_LAYER_MAPPING: [[KeyCode; NUM_ROWS]; NUM_COLS] = [
    [KeyCode::Escape, KeyCode::Tilde, KeyCode::Tab, KeyCode::CapsLock, KeyCode::LeftShift, KeyCode::Fn, KeyCode::Tab, KeyCode::CapsLock, KeyCode::LeftShift, KeyCode::Fn],
    [KeyCode::F1, KeyCode::Num1, KeyCode::Q, KeyCode::A, KeyCode::Empty, KeyCode::LeftCtrl, KeyCode::Tab, KeyCode::CapsLock, KeyCode::LeftShift, KeyCode::Fn],
    [KeyCode::F2, KeyCode::Num2, KeyCode::W, KeyCode::S, KeyCode::Z, KeyCode::LeftAlt, KeyCode::Tab, KeyCode::CapsLock, KeyCode::LeftShift, KeyCode::Fn],
    [KeyCode::F3, KeyCode::Num3, KeyCode::E, KeyCode::D, KeyCode::X, KeyCode::LeftCmd, KeyCode::Tab, KeyCode::CapsLock, KeyCode::LeftShift, KeyCode::Fn],
    [KeyCode::F4, KeyCode::Num4, KeyCode::R, KeyCode::F, KeyCode::C, KeyCode::Empty, KeyCode::Tab, KeyCode::CapsLock, KeyCode::LeftShift, KeyCode::Fn],
    [KeyCode::F5, KeyCode::Num5, KeyCode::T, KeyCode::G, KeyCode::V, KeyCode::Empty, KeyCode::Tab, KeyCode::CapsLock, KeyCode::LeftShift, KeyCode::Fn],
    [KeyCode::Empty, KeyCode::Num6, KeyCode::Y, KeyCode::H, KeyCode::B, KeyCode::Space, KeyCode::Tab, KeyCode::CapsLock, KeyCode::LeftShift, KeyCode::Fn],
    [KeyCode::F6, KeyCode::Num7, KeyCode::U, KeyCode::J, KeyCode::N, KeyCode::Empty, KeyCode::Tab, KeyCode::CapsLock, KeyCode::LeftShift, KeyCode::Fn],
];
/*
#[rustfmt::skip]
pub const NORMAL_LAYER_MAPPING: [[KeyCode; NUM_COLS]; NUM_ROWS] = [
    [KeyCode::Escape, KeyCode::Tab, KeyCode::A, KeyCode::Z, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::Num1, KeyCode::Q, KeyCode::D, KeyCode::Z, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::Num2, KeyCode::W, KeyCode::S, KeyCode::Z, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::Num3, KeyCode::E, KeyCode::H, KeyCode::Z, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::Num4, KeyCode::R, KeyCode::F, KeyCode::Z, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::Num5, KeyCode::Y, KeyCode::G, KeyCode::Z, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::Num6, KeyCode::T, KeyCode::J, KeyCode::Z, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::Num7, KeyCode::U, KeyCode::K, KeyCode::Z, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::Num8, KeyCode::I, KeyCode::Semicolon, KeyCode::Z, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::Num9, KeyCode::O, KeyCode::L, KeyCode::Z, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
];

#[rustfmt::skip]
pub const FN_LAYER_MAPPING: [[KeyCode; NUM_COLS]; NUM_ROWS] = [
    [KeyCode::Escape, KeyCode::Tab, KeyCode::A, KeyCode::Z, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::Num1, KeyCode::Q, KeyCode::D, KeyCode::Z, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::Num2, KeyCode::W, KeyCode::S, KeyCode::Z, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::Num3, KeyCode::E, KeyCode::H, KeyCode::Z, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::Num4, KeyCode::R, KeyCode::F, KeyCode::Z, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::Num5, KeyCode::Y, KeyCode::G, KeyCode::Z, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::Num6, KeyCode::T, KeyCode::J, KeyCode::Z, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::Num7, KeyCode::U, KeyCode::K, KeyCode::Z, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::Num8, KeyCode::I, KeyCode::Semicolon, KeyCode::Z, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
    [KeyCode::Num9, KeyCode::O, KeyCode::L, KeyCode::Z, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty, KeyCode::Empty],
];
*/
