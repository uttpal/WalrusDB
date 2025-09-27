mod memtable;
mod wal;
mod transaction_manager;

use std::io::Error;
use std::sync::Arc;
use crate::transaction_manager::DBEntry;

fn main() -> Result<(), Error>{
    println!("Hello, world!");
    let mut tm = transaction_manager::TransactionManager::new()?;
    tm.replay()?;
    let sample_entry = DBEntry{ key: Arc::new(b"test4".to_vec()), value: Arc::new(b"demo4".to_vec())};
    tm.write(DBEntry { key: sample_entry.key.clone(), value: sample_entry.value.clone()})?;

    let read_entry = tm.read(sample_entry.key.clone())?;
    println!("17 {:?}", String::from_utf8(read_entry.unwrap().to_vec()));
    for entry_result in tm.wal.iter()? {
        let entry = entry_result?;
        println!("20 {:?} {:?}", String::from_utf8(entry.key.to_vec()), String::from_utf8(entry.value.to_vec()))
    }
    Ok(())
}
