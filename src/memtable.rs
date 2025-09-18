use std::collections::BTreeMap;

pub type Key = Vec<u8>;
pub type Value = Vec<u8>;

pub struct Memtable {
    entries: BTreeMap<Key, Value>
}

impl Memtable {
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new()
        }
    }

    pub fn put(&mut self, key:Key, value:Value) {
        self.entries.insert(key, value);
    }

    pub fn get(&self, key:Key) -> Option<Value> {
        self.entries.get(&key).cloned()
    }
}