use std::{collections::HashMap, fs::File, io::Read};

use serde::Deserialize;
use serde_json::{from_value, Map, Value};

#[derive(Clone, Deserialize)]
pub struct Key {
    pub key: String,
    pub action: String,
}

#[derive(Clone)]
pub struct KeyMap {
    pub layout: HashMap<String, Key>,
}

impl KeyMap {
    fn from_serde_map(obj: Map<String, Value>) -> HashMap<String, Key> {
        let mut layout = HashMap::new();

        for (key, value) in obj.iter() {
            let key_map_key_value = value.clone();
            let key_map_key: Key = from_value(key_map_key_value).unwrap();
            layout.insert(key.to_string(), key_map_key);
        }

        layout
    }
    pub fn open(file: Option<String>) -> KeyMap {
        let mut layout = HashMap::new();
        if file.is_none() {
            let mut file = File::open("./a2pi_keymaps/kbmap.json").unwrap();
            let mut data = String::new();
            file.read_to_string(&mut data).unwrap();

            let parsed: Map<String, Value> = serde_json::from_str(&data).unwrap();
            layout = KeyMap::from_serde_map(parsed);
        }
        Self { layout }
    }
    pub fn find_scan_code(self, scan_code: String) -> Option<Key> {
        self.layout.get(&scan_code).cloned()
    }
}
