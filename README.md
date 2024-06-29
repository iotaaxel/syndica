# Syndica

The Task:
The task is to understand the PoH component in the Solana Repository
(https://github.com/solana-labs/solana at commit d0b1f2c).
The task consists of three sub-tasks:
1. Read and understand how Solana’s PoH chain is created
2. Write code that emulates the same logic to create a continuous PoH chain over time while
receiving hashes to mix into the chain. You can write this in any language you choose.
3. Write a short report/blog post on the methods/structures listed on page 2. You can assume
your audience is the rest of the Sig team who understands Rust but not how PoH works or
how it is implemented. You can also use https://excalidraw.com/ to create diagrams.

There are a few main structures which we want to understand more in-depth and how they connect:
- Poh : entry/src/poh.rs
- Entry : entry/src/entry.rs
- PohService : poh/src/poh_service.rs
- PohRecorder: poh/src/poh_recorder.rs
The main functions we want to understand include:
- Poh :: tick(...)
- Poh :: record(...)
- Poh :: hash(...)
- PohRecorder :: record(...)
- PohService :: tick_producer(...)
Include documentation on these structures and functions in the final report. Feel free to
include/explain additional structures/methods which you think are important to understand.
