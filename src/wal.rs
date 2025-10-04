use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, BufReader, Seek, SeekFrom, Write, Read};
use std::sync::Arc;
use crate::memtable::{Key, Value};

pub struct WalEntry {
    pub key: Key,
    pub value: Value,
}

pub struct FileWal {
    path: String,
    writer: BufWriter<File>,
    reader: BufReader<File>,
    position: u64,
}

fn serialize<W: Write>(entry: &WalEntry, mut w: W) -> io::Result<()> {
    let key_len = entry.key.len().to_le_bytes();
    let value_len = entry.value.len().to_le_bytes();
    w.write_all(&key_len)?;
    w.write_all(&value_len)?;
    w.write_all(&entry.key)?;
    w.write_all(&entry.value)?;
    Ok(())
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
    Ok(WalEntry { key: Arc::new(key), value: Arc::new(value)})
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

impl FileWal {
    pub fn open(path: &str) -> io::Result<Self> {
        let file_write_handle= OpenOptions::new().create(true).append(true).read(true).open(path)?;
        let file_read_handle= OpenOptions::new().read(true).open(path)?;
        let position = file_write_handle.metadata()?.len();

        Ok(Self {
            path: path.to_string(),
            writer: BufWriter::new(file_write_handle),
            reader: BufReader::new(file_read_handle),
            position
        })
    }

    pub fn append(&mut self, entry: &WalEntry) -> io::Result<u64> {
        serialize(entry, &mut self.writer)?;
        self.position = self.writer.stream_position()?;

        Ok(self.position)
    }

    pub fn sync(&mut self) -> io::Result<()> {
        // TODO: Implement buffered write
        self.writer.flush()?;
        self.writer.get_mut().sync_all()?;
        Ok(())
    }

    pub fn iter(&mut self) -> io::Result<impl Iterator<Item = io::Result<WalEntry>>> {
        self.reader.seek(SeekFrom::Start(0))?;
        Ok(WalIter { reader: &mut self.reader })
    }

    pub fn truncate(&mut self) -> io::Result<()> {
        // self.writer.get_ref().set_len(0)
        // seek writer and reader to 0
        // reset position
        Ok(())
    }

    pub fn position(&self) -> u64 {
        self.position
    }
}
