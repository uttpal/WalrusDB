# Plan: Rust LSM-Tree Storage Engine

## Milestone 1: Core Write + Read (Active)
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
* Implement WAL Commmit queue
* Introduce opcode in wal
* Implement delete in wal
* Setup AWS account with s3 express one
* Integrate local s3 express one
* Ship logs to s3 express one
* Make sure only primary writes to wal
* Recover Using logs from s3
---

## Milestone 2: Flush to SSTable
- **Components**
    - SSTable writer (serialize memtable → disk)
    - Manifest (tracks SSTables)
- **Features**
    - Flush memtable on size threshold
    - On read: check memtable → SSTables
- **Deliverable**
    - Database survives restarts and scales beyond memory

---

## Milestone 3: Multiple SSTables + Compaction
- **Components**
    - SSTable reader (support multiple files)
    - Compaction engine
    - Bloom filters
- **Features**
    - Merge SSTables in background
    - Drop overwritten and deleted keys
- **Deliverable**
    - Stable performance for large datasets

---

## Milestone 4: Deletes + Updates
- **Components**
    - Tombstone entries
    - Compaction cleanup
- **Features**
    - Correct deletion semantics
    - Overwrites replace old values
- **Deliverable**
    - Correct CRUD semantics

---

## Milestone 5: Concurrency
- **Components**
    - Reader–writer locks
    - Background threads for flush/compaction
- **Features**
    - Parallel reads and writes
- **Deliverable**
    - Safe concurrent KV store

---

## Milestone 6: Transactions
- **Components**
    - Batched WAL entries
- **Features**
    - Atomic multi-operation writes
    - Crash recovery ensures all-or-nothing
- **Deliverable**
    - Basic ACID semantics

---

## Milestone 7: Networking
- **Components**
    - RPC server (gRPC with `tonic`)
- **Features**
    - Remote `put`, `get`, `delete`
- **Deliverable**
    - Usable networked database

---

## Milestone 8: Advanced Features
- **Options**
    - MVCC (multi-version concurrency control)
    - Secondary indexes
    - Query layer (range scans, filters)
    - Replication / sharding
- **Deliverable**
    - Production-grade distributed database
