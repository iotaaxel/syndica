## Overview:

Understand the PoH component in the [Solana Repository](https://github.com/solana-labs/solana) at commit [d0b1f2c](https://github.com/solana-labs/solana/commit/d0b1f2c7c0ac90543ed6935f65b7cfc4673f74da).

The task consists of three sub-tasks:
- [x] Read and understand how Solana’s PoH chain is created
- [ ] Write code that emulates the same logic to create a continuous PoH chain over time while
receiving hashes to mix into the chain. You can write this in any language you choose.
- [ ] Write a short report/blog post on the methods/structures listed on page 2. You can assume
your audience is the rest of the Sig team who understands Rust but not how PoH works or
how it is implemented. You can also use https://excalidraw.com/ to create diagrams.

## Important Objects

#TODO: There are a few main structures which we want to understand more in-depth and how they connect:

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
