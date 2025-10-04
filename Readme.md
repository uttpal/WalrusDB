WalrusDB is distributed database in rust under development.

### Plan
## Milestone 1: Basic Read/Write persistence
- **Components**
    - [x] Memtable (`BTreeMap<Key, Value>`)
    - WAL (append-only log file)
        - [x] Open,
        - Put
            - Serialization/Deserialization
        - iteration
        - Close
- **Features**
    - `put(key, value)`
    - `get(key)`
    - Crash recovery by replaying WAL into memtable
- **Deliverable**
    - Restartable KV store with durability


## Milestone 2: Buffered WAL on s3 express one
* Implement WAL Commit queue
* Introduce opcode in wal
* Implement delete in wal
* Setup AWS account with s3 express one
* Integrate local s3 express one
* Ship logs to s3 express one
* Make sure only primary writes to wal
* Recover Using logs from s3
---
