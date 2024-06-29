## Overview:

Understand the PoH component in the [Solana Repository](https://github.com/solana-labs/solana) at commit [d0b1f2c](https://github.com/solana-labs/solana/commit/d0b1f2c7c0ac90543ed6935f65b7cfc4673f74da).

The task consists of three sub-tasks:
- [x] Read and understand how Solana’s PoH chain is created
- [x] Write code that emulates the same logic to create a continuous PoH chain over time while
receiving hashes to mix into the chain.
- [x] Write a short explanation of the methods/structures to understand them more in-depth and how they connect. 

## Example Run

### Usage
```Rust
cargo run
```

### Output
```Rust
Chain: Node {
    hash_state: "957a9c5209a2148cadfc1de08e95145a86811276759fd19a37de72dcafb94118",
    next: Some(
        Node {
            hash_state: "bde4693e55a336ff81ab238ce20cae1dd9c8ba03b9b8f43963f5569bf3cf5229",
            next: Some(
                Node {
                    hash_state: "4beace8bdcf9b5b74630eaee2e7f501180e46025ca89b05e7e041fbe953d817a",
                    next: Some(
                        Node {
                            hash_state: "293755ab6384e02d9202d483f2f0250100d786e75fdab1b6f3925b2800ece3cb",
                            next: Some(
                                Node {
                                    hash_state: "2435dc0372e12b3f7684fb7093fbe6f6dee79dbff96cc28b1687839ef526e02f",
                                    next: None,
                                },
                            ),
                        },
                    ),
                },
            ),
        },
    ),
}
```

## Important Objects

### Important Structs
- [`Poh`](https://github.com/solana-labs/solana/blob/d0b1f2c7c0ac90543ed6935f65b7cfc4673f74da/entry/src/poh.rs#L10)
    ```Rust
    pub struct Poh {
        pub hash: Hash,
        num_hashes: u64,
        hashes_per_tick: u64,
        remaining_hashes: u64,
        tick_number: u64,
        slot_start_time: Instant,
    }
    ```
    - The `hash` field has the following underlying structure:
        ```Rust
        const HASH_BYTES: usize = 32;
        #[derive(AbiExample)]
        pub struct Hash(pub [u8; HASH_BYTES]);
        ```
        - Calling `Hash::default()` will give bytes in the form `Hash([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])`.
     
    - `num_hashes` is the total number of hashes in the proof of history chain.
    - `hashes_per_tick` is the number of hashes in each tick chain.
    - `remaining_hashes` is the number of remaining hashes in the proof of history chain.
    - `tick_number` is the number of ticks per slot. 
    - `slot_start_time` is from [`std::time::Instant`](https://doc.rust-lang.org/nightly/std/time/struct.Instant.html) and [`Instant::now()`](https://doc.rust-lang.org/nightly/std/time/struct.Instant.html#method.now) represents the current time. 
    
- [`Entry`](https://github.com/solana-labs/solana/blob/d0b1f2c7c0ac90543ed6935f65b7cfc4673f74da/entry/src/entry.rs#L135)
    - The documentation explains the `Entry` struct.
        ```Rust
        #[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq, Clone)]
        pub struct Entry {
            /// The number of hashes since the previous Entry ID.
            pub num_hashes: u64,
        
            /// The SHA-256 hash `num_hashes` after the previous Entry ID.
            pub hash: Hash,
        
            /// An unordered list of transactions that were observed before the Entry ID was
            /// generated. They may have been observed before a previous Entry ID but were
            /// pushed back into this list to ensure deterministic interpretation of the ledger.
            pub transactions: Vec<VersionedTransaction>,
        }
        ```
- [`PohService`](https://github.com/solana-labs/solana/blob/d0b1f2c7c0ac90543ed6935f65b7cfc4673f74da/poh/src/poh_service.rs#L20)
  ```Rust
  pub struct PohService {
    tick_producer: JoinHandle<()>,
  }
  ```
  - The `tick_producer` field uses [`std::thread::JoinHandle`](https://doc.rust-lang.org/std/thread/struct.JoinHandle.html#) to attach to a thread and it can be joined which means the [`join`](https://doc.rust-lang.org/std/thread/struct.JoinHandle.html#method.join) function will wait until the thread is finished.
    
- [`PohRecorder`](https://github.com/solana-labs/solana/blob/d0b1f2c7c0ac90543ed6935f65b7cfc4673f74da/poh/src/poh_recorder.rs#L282)
  ```Rust
  pub struct PohRecorder {
    pub poh: Arc<Mutex<Poh>>,
    tick_height: u64,
    clear_bank_signal: Option<Sender<bool>>,
    start_bank: Arc<Bank>,         // parent slot
    start_tick_height: u64,        // first tick_height this recorder will observe
    tick_cache: Vec<(Entry, u64)>, // cache of entry and its tick_height
    working_bank: Option<WorkingBank>,
    sender: Sender<WorkingBankEntry>,
    poh_timing_point_sender: Option<PohTimingSender>,
    leader_first_tick_height_including_grace_ticks: Option<u64>,
    leader_last_tick_height: u64, // zero if none
    grace_ticks: u64,
    id: Pubkey,
    blockstore: Arc<Blockstore>,
    leader_schedule_cache: Arc<LeaderScheduleCache>,
    ticks_per_slot: u64,
    target_ns_per_tick: u64,
    record_lock_contention_us: u64,
    flush_cache_no_tick_us: u64,
    flush_cache_tick_us: u64,
    send_entry_us: u64,
    tick_lock_contention_us: u64,
    total_sleep_us: u64,
    record_us: u64,
    report_metrics_us: u64,
    ticks_from_record: u64,
    last_metric: Instant,
    record_sender: Sender<Record>,
    leader_bank_notifier: Arc<LeaderBankNotifier>,
    pub is_exited: Arc<AtomicBool>,
  }
  ```

### Main Functions
- [`Poh :: tick(...)`](https://github.com/solana-labs/solana/blob/d0b1f2c7c0ac90543ed6935f65b7cfc4673f74da/entry/src/poh.rs#L90)
  ```Rust
      pub fn tick(&mut self) -> Option<PohEntry> {
            self.hash = hash(self.hash.as_ref()); // hash the current hash value
            self.num_hashes += 1; // increment the number of total hashes
            self.remaining_hashes -= 1; // decrement the number of remaining hashes
    
            // If we are in low power mode then always generate a tick.
            // Otherwise only tick if there are no remaining hashes
            if self.hashes_per_tick != LOW_POWER_MODE && self.remaining_hashes != 0 {
                return None;
            }
    
            let num_hashes = self.num_hashes;
            self.remaining_hashes = self.hashes_per_tick; // the number of remaining hashes in a tick
            self.num_hashes = 0; // the default number of hashes
            self.tick_number += 1; // increment the number of ticks
            // Return a proof of history entry 
            Some(PohEntry {
                num_hashes,
                hash: self.hash,
            })
        }
    }
  ```

- [`Poh :: record(...)`](https://github.com/solana-labs/solana/blob/d0b1f2c7c0ac90543ed6935f65b7cfc4673f74da/entry/src/poh.rs#L74)
  ```Rust
  pub fn record(&mut self, mixin: Hash) -> Option<PohEntry> {
        // If only one hash left
        if self.remaining_hashes == 1 {
            return None; // Caller needs to `tick()` first
        }

        self.hash = hashv(&[self.hash.as_ref(), mixin.as_ref()]); // update the new hash with given hash
        let num_hashes = self.num_hashes + 1; // increment the total number of hashes
        self.num_hashes = 0; // the default number of hashes
        self.remaining_hashes -= 1; // the number of remaining hashes
        // Return a proof of history entry
        Some(PohEntry {
            num_hashes,
            hash: self.hash,
        })
    }
  ```
    - One of the underlying structures is `hashv` which hashes multiple values: 
        ```Rust
        pub fn hashv(&mut self, vals: &[&[u8]]) {
            for val in vals {
                self.hash(val);
            }
        }
        ```
- [`Poh :: hash(...)`](https://github.com/solana-labs/solana/blob/d0b1f2c7c0ac90543ed6935f65b7cfc4673f74da/entry/src/poh.rs#L61)
  ```Rust
  pub fn hash(&mut self, max_num_hashes: u64) -> bool {
        // Get the minimum number of hashes (subtracting by 1 just in case only one hash left)
        let num_hashes = std::cmp::min(self.remaining_hashes - 1, max_num_hashes);
        // Loop through hashes
        for _ in 0..num_hashes {
            // hash the current hash 
            self.hash = hash(self.hash.as_ref());
        }
        self.num_hashes += num_hashes; // increment by the number of hashes
        self.remaining_hashes -= num_hashes; // decrement by the number of hashes

        assert!(self.remaining_hashes > 0); // check that there are available hashes left
        self.remaining_hashes == 1 // Return `true` if caller needs to `tick()` next 
    }
  ```
  
- [`PohRecorder :: record(...)`](https://github.com/solana-labs/solana/blob/d0b1f2c7c0ac90543ed6935f65b7cfc4673f74da/poh/src/poh_recorder.rs#L205)
```Rust
// Returns the index of `transactions.first()` in the slot, if being tracked by WorkingBank
    pub fn record(
        &self,
        bank_slot: Slot, // A [slot](https://docs.rs/solana-sdk/latest/solana_sdk/clock/type.Slot.html) is a  unit of time given to a leader for encoding a block.
        mixin: Hash,
        transactions: Vec<VersionedTransaction>,
    ) -> Result<Option<usize>> {
        // create a new channel so that there is only 1 sender and when it goes out of scope, the receiver fails
        let (result_sender, result_receiver) = unbounded();
        // Attempt to send the record
        let res =
            self.record_sender
                .send(Record::new(mixin, transactions, bank_slot, result_sender));
        if res.is_err() {
            // If the channel is dropped, then the validator is shutting down so return that we are hitting
            //  the max tick height to stop transaction processing and flush any transactions in the pipeline.
            return Err(PohRecorderError::MaxHeightReached);
        }
        // Besides validator exit, this timeout should primarily be seen to affect test execution environments where the various pieces can be shutdown abruptly
        let mut is_exited = false;
        loop {
            // Try to receive the result
            let res = result_receiver.recv_timeout(Duration::from_millis(1000));
            // match on the result
            match res {
                Err(RecvTimeoutError::Timeout) => {
                    if is_exited {
                        return Err(PohRecorderError::MaxHeightReached);
                    } else {
                        // A result may have come in between when we timed out checking this
                        // bool, so check the channel again, even if is_exited == true
                        is_exited = self.is_exited.load(Ordering::SeqCst);
                    }
                }
                Err(RecvTimeoutError::Disconnected) => {
                    return Err(PohRecorderError::MaxHeightReached);
                }
                Ok(result) => {
                    // Return the result if successful
                    return result;
                }
            }
        }
    }
}
```
- [`PohService :: tick_producer(...)`](https://github.com/solana-labs/solana/blob/d0b1f2c7c0ac90543ed6935f65b7cfc4673f74da/poh/src/poh_service.rs#L332)
```Rust
fn tick_producer(
        poh_recorder: Arc<RwLock<PohRecorder>>,
        poh_exit: &AtomicBool,
        ticks_per_slot: u64,
        hashes_per_batch: u64,
        record_receiver: Receiver<Record>,
        target_ns_per_tick: u64,
    ) {
        let poh = poh_recorder.read().unwrap().poh.clone();
        let mut timing = PohTiming::new();
        let mut next_record = None;
        loop {
            let should_tick = Self::record_or_hash(
                &mut next_record,
                &poh_recorder,
                &mut timing,
                &record_receiver,
                hashes_per_batch,
                &poh,
                target_ns_per_tick,
            );
            if should_tick {
                // Lock PohRecorder only for the final hash. record_or_hash will lock PohRecorder for record calls but not for hashing.
                {
                    let mut lock_time = Measure::start("lock");
                    let mut poh_recorder_l = poh_recorder.write().unwrap();
                    lock_time.stop();
                    timing.total_lock_time_ns += lock_time.as_ns();
                    let mut tick_time = Measure::start("tick");
                    poh_recorder_l.tick();
                    tick_time.stop();
                    timing.total_tick_time_ns += tick_time.as_ns();
                }
                timing.num_ticks += 1;

                timing.report(ticks_per_slot);
                if poh_exit.load(Ordering::Relaxed) {
                    break;
                }
            }
        }
    }
```

## Future Work
- Consider adding additional fields like `From`, `To`, and `Txn Fee`. See [etherscan](https://etherscan.io/txs) for an example.
- Consider using multiple threads and asynchronous functions. 
- Add test coverage. 
