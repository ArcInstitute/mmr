use clap::{Parser, ValueEnum};
use minimap2::MapOpt;

#[derive(Parser, Clone, Copy)]
#[clap(next_help_heading = "MAPPING OPTIONS")]
pub struct MappingOptions {
    #[clap(
        short = 'f',
        long,
        help = "Filter out top FLOAT fraction of repetitive minimizers [default = 0.0002]"
    )]
    pub mask_level: Option<f32>,

    #[clap(
        short = 'g',
        long,
        help = "Stop chain elongation if there are no minimizers in INT-bp [default = 10000]"
    )]
    pub max_gap: Option<i32>,

    #[clap(
        short = 'G',
        long,
        help = "Max intron length (effective with -xsplice) [default = 200000]"
    )]
    pub max_gap_ref: Option<i32>,

    #[clap(
        short = 'F',
        long,
        help = "Max fragment length (effective with -xsr or in the fragment mode) [default = 800]"
    )]
    pub max_frag_len: Option<i32>,

    #[clap(short = 'r', long, value_parser = parse_integer_tuple, help = "Chaining/alignment bandwidth and long-join bandwidth [default = 500,20000]")]
    pub bandwidth: Option<(i32, i32)>,

    #[clap(
        short = 'n',
        long,
        help = "Minimal number of minimizers on a chain [default = 3]"
    )]
    pub min_cnt: Option<i32>,

    #[clap(
        short = 'm',
        long,
        help = "Minimal chaining score (matching bases minus log gap penalty) [default = 40]"
    )]
    pub min_chain_score: Option<i32>,

    #[clap(
        short = 'p',
        long,
        help = "Min secondary-to-primary score ratio [default = 0.8]"
    )]
    pub pri_ratio: Option<f32>,

    #[clap(
        short = 'N',
        long,
        help = "Retain at most INT secondary alignments [default = 5]"
    )]
    pub best_n: Option<i32>,

    // Alignment scoring parameters
    #[clap(short = 'A', long, help = "Matching score [default = 2]")]
    pub a: Option<i32>,

    #[clap(
        short = 'B',
        long,
        help = "Mismatch penalty (larger value for lower divergence) [default = 4]"
    )]
    pub b: Option<i32>,

    #[clap(short = 'O', long, value_parser = parse_integer_tuple, help = "Gap open penalties. Format: INT,INT [default = 4,24]")]
    pub gap_open: Option<(i32, i32)>,

    #[clap(short = 'E', long, value_parser = parse_integer_tuple, help = "Gap extension penalties. Format: INT,INT [default = 2,1]")]
    pub gap_ext: Option<(i32, i32)>,

    #[clap(short = 'z', long, value_parser = parse_integer_tuple, help = "Z-drop score and inversion Z-drop score [default = 400,200]")]
    pub zdrop: Option<(i32, i32)>,

    #[clap(
        short = 'u',
        long = "splice-mode",
        help = "How to find canonical splicing sites GT-AG - f:transcript strand; b:both strands; r:reverse strand; n:don't match GT-AG [default = n]"
    )]
    pub splice_mode: Option<SpliceSiteMode>,
}
fn parse_integer_tuple(s: &str) -> Result<(i32, i32), String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return Err("Expected format: INT,INT".to_string());
    }
    Ok((
        parts[0].parse::<i32>().map_err(|e| e.to_string())?,
        parts[1].parse::<i32>().map_err(|e| e.to_string())?,
    ))
}

/// How to find canonical splicing sites GT-AG
#[derive(Clone, Copy, Debug, PartialEq, ValueEnum, Default)]
pub enum SpliceSiteMode {
    /// Don't attempt to match GT-AG (default)
    #[clap(name = "n")]
    #[default]
    None,

    /// Match GT-AG on the forward/transcript strand only
    #[clap(name = "f")]
    Forward,

    /// Match GT-AG on both strands
    #[clap(name = "b")]
    Both,

    /// Match CT-AC on the reverse strand (reverse complement of GT-AG)
    #[clap(name = "r")]
    Reverse,
}
impl SpliceSiteMode {
    pub fn update_mapopt(&self, mapopt: &mut MapOpt) {
        match self {
            Self::None => {
                mapopt.unset_splice_for();
                mapopt.unset_splice_rev();
            }
            Self::Forward => {
                mapopt.set_splice_for();
                mapopt.unset_splice_rev();
            }
            Self::Both => {
                mapopt.set_splice_for();
                mapopt.set_splice_rev();
            }
            Self::Reverse => {
                mapopt.unset_splice_for();
                mapopt.set_splice_rev();
            }
        }
    }
}
