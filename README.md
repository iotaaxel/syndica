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
- [`Entry`](https://github.com/solana-labs/solana/blob/d0b1f2c7c0ac90543ed6935f65b7cfc4673f74da/entry/src/entry.rs#L135)
- [`PohService`](https://github.com/solana-labs/solana/blob/d0b1f2c7c0ac90543ed6935f65b7cfc4673f74da/poh/src/poh_service.rs#L20)
- [`PohRecorder`](https://github.com/solana-labs/solana/blob/d0b1f2c7c0ac90543ed6935f65b7cfc4673f74da/poh/src/poh_recorder.rs#L282)

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