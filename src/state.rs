/*
use crate::{
    drivers::kb::{driver::KbDriver, input::KbDriverInput, state::KbDriverState},
    errors::A2PiError,
};
use mio_serial::SerialStream;
use std::sync::Arc;

use parking_lot::FairMutex;

#[derive(Debug, Clone)]
pub enum State {
    Start,
    Run,
    // Reset,
    // Stop,
}

pub struct A2PiState {
    pub state: State,
    pub kb_driver: KbDriver,
    pub kb_driver_state: Arc<KbDriverState>,
    pub rx_buffer: Vec<u8>,
}

impl Clone for A2PiState {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            kb_driver: self.kb_driver.clone(),
            kb_driver_state: self.kb_driver_state.clone(),
            rx_buffer: self.rx_buffer.clone(),
        }
    }
}

#[cfg_attr(not("std"))]
impl A2PiState {
    pub fn new() -> A2PiState {
        A2PiState {
            state: State::Start,
            kb_driver: KbDriver::init(None),
            kb_driver_state: Arc::new(KbDriverState::reset()),
            rx_buffer: Vec::new(),
        }
    }
    pub fn handler(&mut self, conn: &mut SerialStream, payload: &[u8]) -> Result<(), A2PiError> {
        println!("[{:?}]: {:02X?}", self.state, payload);

        match self.state {
            State::Start => {
                let handshake = self.kb_driver.handshake(conn);
                match handshake {
                    Ok(_) => {
                        self.state = State::Run;
                        println!("Apple II connected!");
                    }
                    Err(e) => {
                        println!("{:02X?} {:?}", payload, e);
                        let _may_fail = self.kb_driver.reset(conn);
                    }
                }
            }
            State::Run => {
                let payload_buffer = {
                    /*
                    if payload[0] == 0x80 {
                        let _may_fail = self.kb_driver.handshake(conn);
                        self.state = State::Start;
                    }
                    */
                    if payload.len() % 3 != 0 || payload[0] == 0x98 {
                        if payload[0] != 0x98 {
                            // a bug in the client assembly that runs on the apple
                            // ii, is inconsistent transmissions of scan(ned) codes
                            //
                            // such can be described as expected a 3 byte vector
                            // but receiving 2 vectors of mismatchd length's that
                            // sum to 3^n bytes.
                            //
                            // this inconsistency can be remedied by buffering
                            // incomplete payloads.
                            for p in payload {
                                self.rx_buffer.push(*p);
                            }
                        } else {
                            println!("malformed kb input!!!");
                            return Ok(());
                        }
                        if self.rx_buffer.len() % 3 != 0 {
                            return Ok(());
                        }
                        let payload = self.rx_buffer.clone();
                        self.rx_buffer = Vec::new();
                        payload
                    } else {
                        payload.to_vec()
                    }
                };

                // payload.len() may exceed 3 indicating an multi key presses
                // so we should chunk each 3 pair and iter over each key press
                let chunks: Vec<&[u8]> = payload_buffer.chunks(3).collect();
                for payload_chunk in chunks {
                    let kb_input = KbDriverInput::from_apple_ii(payload_chunk, &|scan_code| {
                        self.kb_driver.clone().lookup_scan_code(scan_code)
                    });
                    if let Err(e) = kb_input {
                        match e {
                            A2PiError::InvalidKBPayload => {
                                // self.state = State::Start;
                            }
                            A2PiError::InvalidKBInput => {
                                println!("invalid kb input!!!");
                            }
                            A2PiError::InvalidKBModifier => {
                                println!("invalid kb modifier!!!");
                            }
                            _ => {}
                        }
                        self.kb_driver.reset_device();
                        return Ok(());
                    }
                    let kb_inp = kb_input.unwrap().clone().unwrap();
                    self.kb_driver.reset_device();
                    self.kb_driver
                        .emit_to_device(self.kb_driver_state.clone(), kb_inp);
                    /*
                    {
                        self.kb_driver.reset_device();
                        let guard = self.kb_driver_state.try_lock();
                        if let Some(mut kb_driver_state) = guard {

                            (*kb_driver_state).process_input(kb_inp.clone());

                            /*
                                if !kb_driver_state.chained_key_inputs.is_empty() {
                                } else {
                                }
                            */
                        } else {
                            println!("kb_driver_state is locked. unable to handle kb input!!!");
                        }
                    }
                            */
                }
            }
        }
        Ok(())
    }
    pub fn shutdown(&mut self) {
        self.kb_driver.reset_device()
    }
}
*/
