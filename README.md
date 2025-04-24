# mmr

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE.md)
[![Crates.io](https://img.shields.io/crates/d/mmr?color=orange&label=crates.io)](https://crates.io/crates/mmr)

A [minimap2](https://github.com/lh3/minimap2)-based aligner with [BINSEQ](https://github.com/arcinstitute/binseq) file format support (`*.bq` and `*.vbq`).
For converting FASTQ to BINSEQ formats see [bqtools](https://github.com/arcinstitute/bqtools).

This uses the [minimap2-rs](https://github.com/jguhlin/minimap2-rs) library which facilitates raw FFI bindings to the `minimap2` C library.

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

```bash
# map a *.bq file
mmr -x map-pb <library.fa> <query.bq>

# map a *.vbq file
mmr -x map-pb <library.fa> <query.vbq>

# map a *.fq file (supports compressed FASTQ as well)
mmr -x map-pb <library.fa> <query.fq>
```
