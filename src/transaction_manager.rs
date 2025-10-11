use std::{fmt, io};
use std::time::Duration;
use crate::memtable::{Key, Memtable, Value};
use crate::wal::{WalEntry, WalProducer};
use tokio::sync::oneshot;
use tokio::sync::oneshot::{Receiver, Sender};

pub struct DBEntry {
    pub(crate) key: Key,
    pub(crate) value: Value,
}

impl fmt::Display for DBEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DBEntry {} {}", String::from_utf8_lossy(&self.key), String::from_utf8_lossy(&self.value))
    }
}

pub struct TransactionManager {
    pub memtable: Memtable,
    pub wal_producer: WalProducer
}

impl TransactionManager {
    pub fn new(wal_producer: WalProducer) -> Self {
        Self {
            wal_producer,
            memtable: Memtable::new(),
        }
    }

    pub async fn write(&self, entry: DBEntry) -> Result<DBEntry, io::Error> {
        //TODO: Need to change visibility of record after saved in WAL
        self.memtable.put(entry.key.clone(), entry.value.clone());

        let (tx, rx): (Sender<()>, Receiver<()>) = oneshot::channel();
        let wal_entry = WalEntry { key: entry.key.clone(), value: entry.value.clone(), async_waiter: Some(tx)};
        self.wal_producer.append(wal_entry).expect("Buffer full");
        rx.await.expect("TODO: panic message");
        println!("Write Successful {}", entry);

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

    // pub fn replay(&mut self) -> Result<(), io::Error> {
    //     for entry_result in self.wal.iter()? {
    //         let entry = entry_result?;
    //         self.memtable.put(entry.key, entry.value);
    //     }
    //     Ok(())
    // }

}
