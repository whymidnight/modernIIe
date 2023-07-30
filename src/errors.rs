#[derive(Debug, Clone, Copy)]
pub enum A2PiError {
    HandshakeFailureRTSAcquire,
    HandshakeFailureWrite,
    HandshakeFailureRTSClear,

    InvalidKBPayload,
    InvalidKBInput,
    InvalidKBModifier,
}
