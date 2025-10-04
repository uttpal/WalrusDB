### Problem
Syncing WAL I/O over the network on every write hurts performance and increases cost. Buffering WAL writes in memory for a short time amortizes the cost across a batch of transactions.

### Solution
Use a hybrid time-and-count strategy to buffer the WAL; e.g., flush when the number of buffered items > 100 or when no sync has occurred in the past 10 ms.

Multiple independent producer threads add transactions to the WAL buffer and await commit asynchronously. A single consumer thread drains the buffer, commits transactions to durable storage (e.g., S3 Express One), and notifies the waiting threads.

### Implementation Details
Use the Tokio runtime for lightweight threads/tasks. 
Use a lock-free ring buffer as the WAL buffer that multiple threads can write to. 
Alongside each serialized transaction, attach an async waiter object that the producer listens on for commit notification.

#### How does the consumer thread wake up?
Every time a producer adds an item to the ring buffer, it could notify the consumer via an async waiter. 
This would hurt performance because the consumer would wake for every item. 
Instead, use an atomic counter that each producer increments and only notify when it exceeds a threshold.

The consumer thread needs to wait on both a timer and the async notification.

## Technologies used
- Ring buffer: crossbeam `ArrayQueue` as an MPMC queue. TODO: consider replacing with an MPSC queue for better performance.
- Async waiter: `tokio::sync::oneshot`
- Threads: Tokio

### Implementation Plan:
1. Implement ring buffer in wal
2. Implement producer side
3. Implement Consumer side
