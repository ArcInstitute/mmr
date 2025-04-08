# mmr

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE.md)
[![Crates.io](https://img.shields.io/crates/d/mmr?color=orange&label=crates.io)](https://crates.io/crates/mmr)

A [minimap2](https://github.com/lh3/minimap2)-based aligner with [binseq](https://github.com/arcinstitute/binseq) and [vbinseq](https://github.com/arcinstitute/vbinseq) support.
For converting FASTQ to BINSEQ formats see [bqtools](https://github.com/arcinstitute/bqtools).

## Installation

`mmr` is written in rust and deployed with [`cargo`](https://rustup.rs/).

```rust
# install binary from cargo
cargo install mmr

# validate installation
mmr --version
```

## Usage

`mmr` follows the same (or similar) CLI as the original [`minimap2`](https://github.com/lh3/minimap2) binary.
