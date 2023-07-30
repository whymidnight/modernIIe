use core::{convert::Infallible, ops::Deref};

use alloc::vec;
use alloc::vec::Vec;
use cortex_m::delay::Delay;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use rp2040_hal::gpio::PullDownInput;
use usbd_hid::descriptor::KeyboardReport;

use crate::drivers::no_std::kb::{driver::KbDriver, input::Modifiers};

use super::{debounce::Debounce, key_codes::KeyCode, key_mapping};

#[derive(Clone, Copy)]
pub struct KeyScan<const NUM_MODS: usize, const NUM_ROWS: usize, const NUM_COLS: usize> {
    matrix: [[bool; NUM_ROWS]; NUM_COLS],
    mods: [bool; NUM_MODS],
}

pub enum KeyScanDecoder {
    Modifiers(Vec<u8>),
    Characters(Vec<u8>),
}

impl Into<Vec<u8>> for KeyScanDecoder {
    fn into(self) -> Vec<u8> {
        match self {
            Self::Modifiers(modifiers) => modifiers,
            Self::Characters(characters) => characters,
        }
    }
}

/*
impl<const NUM_MODS: usize, const NUM_ROWS: usize, const NUM_COLS: usize> Deref for KeyScan<NUM_ROWS, NUM_COLS> {
    type Target = [[bool; NUM_ROWS]; NUM_COLS];

    fn deref(&self) -> &Self::Target {
        &self.matrix
    }
}
*/

impl<const NUM_MODS: usize, const NUM_ROWS: usize, const NUM_COLS: usize>
    KeyScan<NUM_MODS, NUM_ROWS, NUM_COLS>
{
    pub fn into_decoder(self) -> (KeyScanDecoder, KeyScanDecoder) {
        let modifiers = self
            .mods
            .iter()
            .enumerate()
            .fold(vec![], |mut acc, (modifier, key)| {
                if *key {
                    let scan_code = match modifier {
                        0 => Modifiers::ClosedApple.into(), // SW1 ::  Closed Apple
                        1 => Modifiers::OpenApple.into(),   // SW0 :: Open Apple
                        2 => Modifiers::Control.into(),     // Control
                        3 => Modifiers::Reset.into(),       // RESET
                        4 => Modifiers::Shift.into(),       // Shift
                        _ => 0x0,
                    };

                    acc.push(scan_code);
                }
                acc
            });

        let characters =
            self.matrix
                .iter()
                .enumerate()
                .fold(vec![], |mut acc, (col, matrix_col)| {
                    for (row, matrix_row) in matrix_col.iter().enumerate() {
                        if *matrix_row {
                            acc.push(((col * 16) + row) as u8);
                        }
                    }
                    acc
                });

        (
            KeyScanDecoder::Modifiers(modifiers),
            KeyScanDecoder::Characters(characters),
        )
    }
    pub fn scan(
        modifiers: (
            &[&dyn InputPin<Error = Infallible>],
            &[&dyn InputPin<Error = Infallible>],
        ),
        rows: &[&dyn InputPin<Error = Infallible>],
        columns: &mut [&mut dyn embedded_hal::digital::v2::OutputPin<Error = Infallible>],
        delay: &mut Delay,
        debounce: &mut Debounce<NUM_MODS, NUM_ROWS, NUM_COLS>,
    ) -> Self {
        let mut raw_matrix = [[false; NUM_ROWS]; NUM_COLS];
        let mut raw_modifiers = [false; NUM_MODS];

        let (input_modifiers, output_modifiers) = modifiers;
        for (key, gpio_key) in input_modifiers.iter().enumerate() {
            raw_modifiers[key] = gpio_key.is_high().unwrap();
        }
        for (key, gpio_key) in output_modifiers.iter().enumerate() {
            raw_modifiers[key + input_modifiers.len()] = gpio_key.is_low().unwrap();
        }

        for (col, (gpio_col, matrix_col)) in
            columns.iter_mut().zip(raw_matrix.iter_mut()).enumerate()
        {
            gpio_col.set_high().unwrap();
            delay.delay_us(60);

            for (row, (gpio_row, matrix_row)) in rows.iter().zip(matrix_col.iter_mut()).enumerate()
            {
                *matrix_row = gpio_row.is_high().unwrap();
            }

            gpio_col.set_low().unwrap();
            delay.delay_us(60);
        }

        /*
         */
        let (mods, matrix) = debounce.report_and_tick(&raw_modifiers, &raw_matrix);
        Self { mods, matrix }
    }
}

impl<const NUM_MODS: usize, const NUM_ROWS: usize, const NUM_COLS: usize>
    From<KeyScan<NUM_MODS, NUM_ROWS, NUM_COLS>> for Vec<Vec<u8>>
{
    fn from(value: KeyScan<NUM_MODS, NUM_ROWS, NUM_COLS>) -> Self {
        let mut matrix = value.matrix.clone();

        let scan_code_matrix = matrix.iter_mut().enumerate().map(|(col, matrix_col)| {
            return matrix_col
                .iter_mut()
                .enumerate()
                .map(|(row, matrix_row)| {
                    if *matrix_row {
                        ((col * 16) + row) as u8
                    } else {
                        0
                    }
                })
                .collect();
        });

        let scan_codes = scan_code_matrix.collect();

        scan_codes
    }
}

/*
impl<const NUM_MODS: usize, const NUM_ROWS: usize, const NUM_COLS: usize>
    From<KeyScan<NUM_MODS, NUM_ROWS, NUM_COLS>> for Vec<KeyboardReport>
{
    fn from(scan: KeyScan<NUM_MODS, NUM_ROWS, NUM_COLS>) -> Self {
        let mut keycodes = vec![];
        let mut modifier = 0;

        // First scan for any function keys being pressed
        let layer_mapping = key_mapping::NORMAL_LAYER_MAPPING;

        let mut mods_mapping: Vec<u8> = [].to_vec();
        for (idx, (key_pressed, mapping_key)) in
            scan.mods.iter().zip(mods_mapping.iter_mut()).enumerate()
        {
            if *key_pressed {
                *mapping_key = match idx {
                    0 => 0x80, // SW1 ::  Closed Apple
                    1 => 0x40, // SW0 :: Open Apple
                    2 => 0x20, // Control
                    3 => 0x40, // RESET
                    4 => 0x40, // Shift
                    _ => 0x0,
                }
            }
        }

        /*
         */
        let mut key_codes: Vec<u8> = Vec::new();
        let generate = |col: usize, row: usize| {
            let _a = 2;
            key_codes.push(1);
        };

        // Second scan to generate the correct keycodes given the activated key map
        for (col, matrix_column) in scan.matrix.iter().enumerate() {
            for (row, key_pressed) in matrix_column.iter().enumerate() {
                if *key_pressed {
                    generate(col, row);
                }
            }
        }

        // print matrix in hex
        /*
        defmt::info!("---");
        for (i, d) in raw_matrix.iter().enumerate() {
            let nums: Vec<u8> = d.iter().map(|&x| x.into()).collect();
            defmt::info!("{} ::: {}", i, nums.as_slice());
        }
        defmt::info!("---");
        */

        vec![KeyboardReport {
            modifier,
            reserved: 0,
            leds: 0,
            keycodes,
        }]
    }
}
*/
