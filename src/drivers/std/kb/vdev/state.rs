use enigo::Key;

#[derive(Clone)]
pub enum VdevDeviceKeyEvent {
    KeyDown(Key),
}

impl VdevDeviceKeyEvent {
    pub fn record_key_down(key: Key) -> VdevDeviceKeyEvent {
        VdevDeviceKeyEvent::KeyDown(key)
    }
}

#[derive(Clone)]
pub struct VdevDeviceState {
    pub active_keys_down: Vec<VdevDeviceKeyEvent>,
    // do not need to record keys_up
}

impl VdevDeviceState {
    pub fn init() -> VdevDeviceState {
        Self {
            active_keys_down: Vec::new(),
        }
    }
    pub fn record_key_event(&mut self, event: VdevDeviceKeyEvent) {
        self.active_keys_down.push(event)
    }
}
