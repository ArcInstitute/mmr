use anyhow::Result;
use binseq::MmapReader;
use clap::Parser;

mod align;
mod cli;
mod index;

use align::ParallelAlignment;
use cli::Cli;
use index::build_index;

fn main() -> Result<()> {
    let args = Cli::parse();

    let index = build_index(
        &args.io_options.index_path,
        args.mapping_options,
        args.index_options,
        args.run_options.n_threads,
    )?;

    let aligner = ParallelAlignment::new(index, args.io_options.output_path)?;
    let reader = MmapReader::new(&args.io_options.query_path)?;

    eprintln!("Processing reads...");
    let start = std::time::Instant::now();
    reader.process_parallel(aligner, args.run_options.n_threads)?;
    let duration = std::time::Instant::now().duration_since(start);
    eprintln!("Mapping processed in {:?}", duration);

    Ok(())
}
