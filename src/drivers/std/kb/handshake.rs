use hex::FromHex;
use mio_serial::{SerialPort, SerialStream};
use std::io::Write;

use crate::errors::A2PiError;

pub fn handshake(conn: &mut SerialStream) -> Result<(), A2PiError> {
    // acquire RTS
    let rts = conn.write_request_to_send(true);
    if rts.is_err() {
        return Err(A2PiError::HandshakeFailureRTSAcquire);
    }

    // write 0x81
    let ack = <[u8; 1]>::from_hex("81").unwrap();
    let ack_write = conn.write(&ack);
    if ack_write.is_err() {
        return Err(A2PiError::HandshakeFailureWrite);
    }

    // clear RTS
    let rts = conn.write_request_to_send(false);
    if rts.is_err() {
        return Err(A2PiError::HandshakeFailureRTSClear);
    }

    Ok(())
}
pub fn reset(conn: &mut SerialStream) -> Result<(), A2PiError> {
    // acquire RTS
    let rts = conn.write_request_to_send(true);
    if rts.is_err() {
        return Err(A2PiError::HandshakeFailureRTSAcquire);
    }

    // write 0x80
    let ack_write = conn.write(&[0x80]);
    if ack_write.is_err() {
        return Err(A2PiError::HandshakeFailureWrite);
    }

    // clear RTS
    let rts = conn.write_request_to_send(false);
    if rts.is_err() {
        return Err(A2PiError::HandshakeFailureRTSClear);
    }

    Ok(())
}
