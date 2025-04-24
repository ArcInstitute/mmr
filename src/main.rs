use std::io::Write;
use std::time::Instant;

use anyhow::Result;
use binseq::{BinseqReader, ParallelReader};
use clap::Parser;

mod align;
mod cli;
mod index;
mod io;
mod stats;

use align::ParallelAlignment;
use cli::Cli;
use index::build_index;
use io::{transparent_reader, transparent_writer};
use paraseq::{fastq, parallel::ParallelReader as FastqParallelReader};
use stats::Runtime;

fn report_runtime(
    program_start: Instant,
    map_start: Instant,
    num_records: usize,
    path: Option<&str>,
) -> Result<()> {
    let stats = Runtime::new(program_start, map_start, num_records);
    let mut wtr = transparent_writer(path)?;
    serde_json::to_writer_pretty(&mut wtr, &stats)?;
    wtr.flush()?;
    Ok(())
}

fn process_fastq(
    aligner: ParallelAlignment,
    query_path: &str,
    n_threads: usize,
    start_time: Instant,
    log_path: Option<&str>,
) -> Result<()> {
    let stream = transparent_reader(query_path)?;
    let reader = fastq::Reader::new(stream);
    reader.process_parallel(aligner.clone(), n_threads)?;
    aligner.finish_pbar();
    report_runtime(
        start_time,
        aligner.start_time(),
        aligner.num_records(),
        log_path,
    )
}

fn process_binseq(
    aligner: ParallelAlignment,
    query_path: &str,
    n_threads: usize,
    start_time: Instant,
    log_path: Option<&str>,
) -> Result<()> {
    let reader = BinseqReader::new(query_path)?;
    reader.process_parallel(aligner.clone(), n_threads)?;
    aligner.finish_pbar();
    report_runtime(
        start_time,
        aligner.start_time(),
        aligner.num_records(),
        log_path,
    )
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let start_time = Instant::now();
    let index = build_index(
        &args.io_options.index_path,
        args.mapping_options,
        args.index_options,
        args.run_options.n_threads(),
        args.run_options.show_options,
    )?;
    let aligner = ParallelAlignment::new(index, args.io_options.output_path)?;

    let query_path = &args.io_options.query_path;
    if query_path.ends_with(".bq") || query_path.ends_with(".vbq") {
        process_binseq(
            aligner,
            query_path,
            args.run_options.n_threads(),
            start_time,
            args.run_options.log_path.as_deref(),
        )
    } else {
        process_fastq(
            aligner,
            query_path,
            args.run_options.n_threads(),
            start_time,
            args.run_options.log_path.as_deref(),
        )
    }
}
