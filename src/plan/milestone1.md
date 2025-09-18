## Milestone 1: Core Write + Read
- **Components**
    - WAL (append-only log file)
    - Memtable (`BTreeMap<Key, Value>`)
- **Features**
    - `put(key, value)`
    - `get(key)`
    - Crash recovery by replaying WAL into memtable
- **Deliverable**
    - Restartable KV store with durability

---

- [ ] Create Memtable with write api