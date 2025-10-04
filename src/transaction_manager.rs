use std::io;
use crate::memtable::{Key, Memtable, Value};
use crate::wal::{Wal, WalEntry};
use tokio::sync::oneshot;
use tokio::sync::oneshot::{Receiver, Sender};

pub struct DBEntry {
    pub(crate) key: Key,
    pub(crate) value: Value,
}

pub struct TransactionManager {
    pub memtable: Memtable,
    pub wal: Wal
}

impl TransactionManager {
    pub fn new() -> io::Result<Self> {
        let wal = Wal::open("./wal.txt")?;

        Ok(Self {
            memtable: Memtable::new(),
            wal
        })
    }

    pub async fn write(&mut self, entry: DBEntry) -> Result<DBEntry, io::Error> {
        //TODO: Need to change visibility of record after saved in WAL
        self.memtable.put(entry.key.clone(), entry.value.clone());

        let (tx, rx): (Sender<()>, Receiver<()>) = oneshot::channel();
        let wal_entry = WalEntry { key: entry.key.clone(), value: entry.value.clone(), async_waiter: Some(tx)};
        self.wal.append(wal_entry).expect("Buffer full");
        rx.await.expect("TODO: panic message");

        Ok(DBEntry {
            key: entry.key.clone(),
            value: entry.value.clone(),
        })
    }

    pub fn read(&self, key: Key) -> Result<Option<Value>, io::Error> {
        Ok(self.memtable.get(key))
    }

    pub fn delete(&mut self, key: Key) -> Result<(Option<Value>), io::Error> {
        Ok(self.memtable.delete(key))
    }

    pub fn replay(&mut self) -> Result<(), io::Error> {
        for entry_result in self.wal.iter()? {
            let entry = entry_result?;
            self.memtable.put(entry.key, entry.value);
        }
        Ok(())
    }

}
