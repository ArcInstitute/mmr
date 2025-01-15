use clap::Parser;

#[derive(Parser, Clone, Copy)]
#[clap(next_help_heading = "RUN OPTIONS")]
pub struct RunOptions {
    #[clap(short = 'T', long, default_value = "1")]
    pub n_threads: usize,
}
