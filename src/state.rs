use std::collections::HashMap;

// KeyValueStore definiert
#[derive(Clone)] // Automatische Implementierung des Traits `Clone`
pub struct KeyValueStore {
    pub inner_store: HashMap<String, String>,
}

impl KeyValueStore {
    // Konstruktor für KeyValueStore
    pub fn new() -> KeyValueStore {
        KeyValueStore {
            inner_store: HashMap::new(),
        }
    }

    // Methode zum Einfügen von Schlüsseln und Werten
    pub fn insert(&mut self, key: &str, value: String) {
        self.inner_store.insert(key.to_string(), value);
    }
}