use std::io::Read;

use anyhow::Result;
use binseq::MmapReader;
use clap::Parser;

mod align;
mod cli;
mod index;

use align::ParallelAlignment;
use cli::Cli;
use index::build_index;
use paraseq::{fastq, parallel::ParallelReader};

fn transparent_reader(input: &str) -> Result<Box<dyn Read + Send>> {
    let (stream, _comp) = niffler::send::from_path(input)?;
    Ok(stream)
}

fn process_fastq(aligner: ParallelAlignment, query_path: &str, n_threads: usize) -> Result<()> {
    let stream = transparent_reader(query_path)?;
    let reader = fastq::Reader::new(stream);

    eprintln!("Processing FASTQ records...");
    let start = std::time::Instant::now();
    reader.process_parallel(aligner, n_threads)?;
    let duration = start.elapsed();
    eprintln!("Mapping processed in {:?}", duration);
    Ok(())
}

fn process_binseq(aligner: ParallelAlignment, query_path: &str, n_threads: usize) -> Result<()> {
    let reader = MmapReader::new(query_path)?;

    eprintln!("Processing BINSEQ records...");
    let start = std::time::Instant::now();
    reader.process_parallel(aligner, n_threads)?;
    let duration = start.elapsed();
    eprintln!("Mapping processed in {:?}", duration);
    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let index = build_index(
        &args.io_options.index_path,
        args.mapping_options,
        args.index_options,
        args.run_options.n_threads(),
    )?;
    let aligner = ParallelAlignment::new(index, args.io_options.output_path)?;

    let query_path = &args.io_options.query_path;
    if query_path.ends_with(".bq") {
        process_binseq(aligner, query_path, args.run_options.n_threads())
    } else {
        process_fastq(aligner, query_path, args.run_options.n_threads())
    }
}
