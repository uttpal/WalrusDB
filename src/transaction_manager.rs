use std::io;
use crate::memtable::{Key, Memtable, Value};
use crate::wal::{FileWal, WalEntry};

pub struct DBEntry {
    pub(crate) key: Key,
    pub(crate) value: Value,
}

pub struct TransactionManager {
    pub memtable: Memtable,
    pub wal: FileWal
}

impl TransactionManager {
    pub fn new() -> io::Result<Self> {
        let wal = FileWal::open("./wal.txt")?;

        Ok(Self {
            memtable: Memtable::new(),
            wal
        })
    }

    pub fn write(&mut self, entry: DBEntry) -> Result<DBEntry, io::Error> {
        self.memtable.put(entry.key.clone(), entry.value.clone());
        let mut wal_entry = WalEntry { key: entry.key.clone(), value: entry.value.clone()};
        self.wal.append(&mut wal_entry)?;
        self.wal.sync()?;
        Ok(DBEntry {
            key: entry.key.clone(),
            value: entry.value.clone(),
        })
    }

    pub fn read(&self, key: Key) -> Result<Option<Value>, io::Error> {
        Ok(self.memtable.get(key))
    }

    pub fn replay(&mut self) -> Result<(), io::Error> {
        for entry_result in self.wal.iter()? {
            let entry = entry_result?;
            self.memtable.put(entry.key, entry.value)
        }
        Ok(())
    }

}
