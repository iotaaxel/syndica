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
  - #TODO: ....

### Main Functions
- [`Poh :: tick(...)`](https://github.com/solana-labs/solana/blob/d0b1f2c7c0ac90543ed6935f65b7cfc4673f74da/entry/src/poh.rs#L90)
- [`Poh :: record(...)`](https://github.com/solana-labs/solana/blob/d0b1f2c7c0ac90543ed6935f65b7cfc4673f74da/entry/src/poh.rs#L74)
- [`Poh :: hash(...)`](https://github.com/solana-labs/solana/blob/d0b1f2c7c0ac90543ed6935f65b7cfc4673f74da/entry/src/poh.rs#L61)
- [`PohRecorder :: record(...)`](https://github.com/solana-labs/solana/blob/d0b1f2c7c0ac90543ed6935f65b7cfc4673f74da/poh/src/poh_recorder.rs#L205)
- [`PohService :: tick_producer(...)`](https://github.com/solana-labs/solana/blob/d0b1f2c7c0ac90543ed6935f65b7cfc4673f74da/poh/src/poh_service.rs#L332)

## Future Work
- Consider adding additional fields like `From`, `To`, and `Txn Fee`. See [etherscan](https://etherscan.io/txs) for an example.
- Consider using multiple threads and asynchronous functions. 
- Add test coverage. 
