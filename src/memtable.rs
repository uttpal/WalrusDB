use std::collections::BTreeMap;
use std::sync::Arc;

pub type Key = Arc<Vec<u8>>;
pub type Value = Arc<Vec<u8>>;
const TOMBSTONE: &[u8] = b"__DEL__";

pub struct Memtable {
    entries: BTreeMap<Key, Value>
}

impl Memtable {
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new()
        }
    }

    pub fn put(&mut self, key:Key, value:Value) -> bool {
        let val = self.entries.insert(key, value);
        val.is_none()
    }

    pub fn get(&self, key:Key) -> Option<Value> {
        // check if value is not tombstone
        let val = self.entries.get(&key);
        match val {
            Some(val) if *val.as_ref() == TOMBSTONE.to_vec() => None,
            Some(val) => Some(val.clone()),
            None => None,
        }
    }

    pub fn delete(&mut self, key:Key) -> Option<Value> {
        self.entries.insert(key, Arc::from(TOMBSTONE.to_vec()))
    }
}