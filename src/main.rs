mod memtable;
mod wal;
mod transaction_manager;
mod wal_store;

use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinSet;
use crate::transaction_manager::{DBEntry, TransactionManager};
use crate::wal::WalContainer;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> std::io::Result<()> {
    let mut wal_container = WalContainer::new(10000, Duration::from_secs(10));
    let tm = Arc::new(TransactionManager::new(wal_container.producer));

    tokio::spawn(async move { wal_container.consumer.start().await });

    let mut set = JoinSet::new();

    for id in 0..8 {
        for n in 0..100 {
            let key = Arc::new(format!("k:{id}:{n}").into_bytes());
            let val = Arc::new(format!("v:{id}:{n}").into_bytes());
            let tm_ref = Arc::clone(&tm);

            set.spawn(async move {
                tm_ref.write(DBEntry {
                    key: key.clone(),
                    value: val.clone()
                }).await.unwrap()
            });
        }
    }

    while let Some(res) = set.join_next().await {
        let _ = res;
    }    Ok(())
}
