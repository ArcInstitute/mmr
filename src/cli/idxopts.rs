use clap::Parser;

use super::PresetWrapper;

#[derive(Parser, Clone, Copy)]
#[clap(next_help_heading = "INDEX OPTIONS")]
pub struct IndexOptions {
    #[clap(
        short,
        long,
        default_value = "15",
        help = "k-mer size (no larger than 28)"
    )]
    pub kmer_size: i16,

    #[clap(short, long, default_value = "10", help = "minimizer window size")]
    pub window_size: i16,

    /// Preset to use when aligning reads
    #[clap(short = 'x', long)]
    pub preset: PresetWrapper,
}
