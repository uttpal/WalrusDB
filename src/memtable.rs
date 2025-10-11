use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

pub type Key = Arc<Vec<u8>>;
pub type Value = Arc<Vec<u8>>;
const TOMBSTONE: &[u8] = b"__DEL__";

pub struct Memtable {
    entries: RwLock<BTreeMap<Key, Value>>,
}

impl Memtable {
    pub fn new() -> Self {
        Self {
            // TODO: replace with lock free ds
            entries: RwLock::new(BTreeMap::new()),
        }
    }

    pub fn put(&self, key:Key, value:Value) -> bool {
        let mut entries = self.entries.write().expect("failed to write to memtable");
        entries.insert(key, value).is_none()
    }

    pub fn get(&self, key:Key) -> Option<Value> {
        // check if value is not tombstone
        let entries = self.entries.read().expect("failed to read from memtable");
        let val = entries.get(&key);

        match val {
            Some(val) if *val.as_ref() == TOMBSTONE.to_vec() => None,
            Some(val) => Some(val.clone()),
            None => None,
        }
    }

    pub fn delete(&mut self, key:Key) -> Option<Value> {
        let mut entries = self.entries.write().expect("failed to write to memtable");
        entries.insert(key, Arc::from(TOMBSTONE.to_vec()))
    }
}