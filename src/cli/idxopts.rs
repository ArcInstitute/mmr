use clap::Parser;

use super::PresetWrapper;

#[derive(Parser, Clone, Copy)]
#[clap(next_help_heading = "INDEX OPTIONS")]
pub struct IndexOptions {
    #[clap(short, long, help = "k-mer size (no larger than 28) [default: 15]")]
    pub kmer_size: Option<i16>,

    #[clap(
        short,
        long,
        default_value = "10",
        help = "minimizer window size [default: 10]"
    )]
    pub window_size: Option<i16>,

    /// Preset to use when aligning reads
    #[clap(short = 'x', long)]
    pub preset: PresetWrapper,
}
