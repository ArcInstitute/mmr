use clap::Parser;

#[derive(Parser)]
#[clap(next_help_heading = "INPUT FILE OPTIONS")]
pub struct IoOptions {
    #[clap(help = "Path to the file to index")]
    pub index_path: String,
    #[clap(help = "Path to the binseq file to query")]
    pub query_path: String,
    #[clap(short, long, help = "Path to the output file [default: stdout]")]
    pub output_path: Option<String>,
}
