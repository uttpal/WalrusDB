use std::fmt;
use std::io::{self, BufWriter, BufReader, Seek, SeekFrom, Write, Read, Cursor};
use std::sync::Arc;
use std::time::Duration;
use crate::memtable::{Key, Value};
use crossbeam_queue::ArrayQueue;
use tokio::sync::{oneshot, Notify};


#[derive(Debug)]
pub struct WalEntry {
    pub key: Key,
    pub value: Value,
    pub async_waiter: Option<oneshot::Sender<()>>,
}

impl fmt::Display for WalEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let key_str = String::from_utf8_lossy(&self.key);
        let value_str = String::from_utf8_lossy(&self.value);
        write!(f, "WalEntry {{ key: {}, value: {}, async_waiter: {} }}",
               key_str, value_str,
               if self.async_waiter.is_some() { "Some(_)" } else { "None" })
    }
}


pub struct WalConsumer {
    ring_buffer: Arc<ArrayQueue<WalEntry>>,
    notify: Arc<Notify>,
    flush_interval: Duration
}

pub struct WalProducer{
    ring_buffer: Arc<ArrayQueue<WalEntry>>,
    notify: Arc<Notify>,
}

pub struct WalContainer {
    pub producer: WalProducer,
    pub consumer: WalConsumer,
}

impl WalContainer{
    pub fn new(capacity: usize, flush_interval: Duration) -> Self {
        let queue = Arc::new(ArrayQueue::new(capacity));
        let notify = Arc::new(Notify::new());
        Self {
            producer: WalProducer::new(queue.clone(), notify.clone()),
            consumer: WalConsumer::new(queue, notify.clone(), flush_interval),
        }
    }
}

fn serialize(entry: &WalEntry) -> Vec<u8> {
    let mut buf = Vec::with_capacity(
        8 + entry.key.len() + entry.value.len(),
    );
    buf.extend_from_slice(&(entry.key.len() as u64).to_le_bytes());
    buf.extend_from_slice(&(entry.value.len() as u64).to_le_bytes());
    buf.extend_from_slice(&entry.key);
    buf.extend_from_slice(&entry.value);
    buf
}

fn deserialize<R: Read>(mut r: R) -> io::Result<WalEntry> {
    let mut buf4 = [0u8; 8];
    r.read_exact(&mut buf4)?;
    let key_len = usize::from_le_bytes(buf4);
    r.read_exact(&mut buf4)?;
    let value_len = usize::from_le_bytes(buf4);
    let mut key = vec![0;key_len];
    let mut value = vec![0;value_len];

    r.read_exact(&mut key)?;
    r.read_exact(&mut value)?;
    Ok(WalEntry { key: Arc::new(key), value: Arc::new(value), async_waiter: None })
}

fn deserialize_from_bytes(bytes: Vec<u8>) -> io::Result<WalEntry> {
    let mut cursor = Cursor::new(bytes);
    deserialize(&mut cursor)
}

pub struct WalIter<'a, R: Read> {
    reader: &'a mut R
}

impl<'a, R: Read> Iterator for WalIter<'a, R> {
    type Item = io::Result<WalEntry>;
    fn next(&mut self) -> Option<Self::Item> {
        let entry_result = deserialize(&mut self.reader);
        // TODO: Need better error handling while reading, all errors are not EOF
        if let Ok(entry) = entry_result {
            return Some(Ok(entry));
        }
        None
    }
}

impl WalProducer {
    pub fn new(ring_buffer: Arc<ArrayQueue<WalEntry>>, notify: Arc<Notify>) -> Self {
        Self {
            ring_buffer,
            notify,
        }
    }

    pub fn append(&self, entry: WalEntry) -> Result<(), WalEntry> {
        self.ring_buffer.push(entry)
    }
}

impl WalConsumer {
    pub fn new(ring_buffer: Arc<ArrayQueue<WalEntry>>, notify: Arc<Notify>, flush_interval: Duration) -> Self {
        Self {
            ring_buffer,
            notify,
            flush_interval
        }
    }

    // TODO: Move to Wal Reader
    // pub fn iter(&mut self) -> io::Result<impl Iterator<Item = io::Result<WalEntry>>> {
    //     self.reader.seek(SeekFrom::Start(0))?;
    //     Ok(WalIter { reader: &mut self.reader })
    // }

    pub fn truncate(&mut self) -> io::Result<()> {
        // self.writer.get_ref().set_len(0)
        // seek writer and reader to 0
        // reset position
        Ok(())
    }

    // pub fn position(&self) -> u64 {
    //     self.position
    // }

    pub async fn start(&mut self) {
        loop {
            tokio::select! {
                _ = self.notify.notified() => {
                    // Woken by producer on count reach
                },
                _ = tokio::time::sleep(self.flush_interval) => {
                    // Woken by timeout
                }
            }
            let mut wal_batch = Vec::new();
            while let Some(wal_entry) = self.ring_buffer.pop() {
                wal_batch.push(wal_entry);
            }

            if wal_batch.len() == 0 {
                continue;
            }
            //TODO: serialize before adding to wal buffer to maximize single threaded perf on consumer
            let serialized_batch = wal_batch.iter().map(|entry| serialize(entry)).collect::<Vec<Vec<u8>>>();
            for entry in wal_batch {
                //TODO: Write to s3
                println!("WalEntry: {:?}", entry);
                if let Some(waiter) = entry.async_waiter {
                    let _ = waiter.send(());
                }
            }
        }

    }
}
