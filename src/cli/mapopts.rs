use clap::Parser;

#[derive(Parser, Clone, Copy)]
#[clap(next_help_heading = "MAPPING OPTIONS")]
pub struct MappingOptions {
    #[clap(
        short = 'f',
        long,
        default_value = "0.0002",
        help = "Filter out top FLOAT fraction of repetitive minimizers"
    )]
    pub mask_level: f32,

    #[clap(
        short = 'g',
        long,
        default_value = "5000",
        help = "Stop chain elongation if there are no minimizers in INT-bp"
    )]
    pub max_gap: i32,

    #[clap(
        short = 'G',
        long,
        default_value = "200000",
        help = "Max intron length (effective with -xsplice)"
    )]
    pub max_gap_ref: i32,

    #[clap(
        short = 'F',
        long,
        default_value = "800",
        help = "Max fragment length (effective with -xsr or in the fragment mode)"
    )]
    pub max_frag_len: i32,

    #[clap(short = 'r', long, value_parser = parse_integer_tuple, default_value = "500,20000", help = "Chaining/alignment bandwidth and long-join bandwidth")]
    pub bandwidth: (i32, i32),

    #[clap(
        short = 'n',
        long,
        default_value = "3",
        help = "Minimal number of minimizers on a chain"
    )]
    pub min_cnt: i32,

    #[clap(
        short = 'm',
        long,
        default_value = "40",
        help = "Minimal chaining score (matching bases minus log gap penalty)"
    )]
    pub min_chain_score: i32,

    #[clap(
        short = 'p',
        long,
        default_value = "0.8",
        help = "Min secondary-to-primary score ratio"
    )]
    pub pri_ratio: f32,

    #[clap(
        short = 'N',
        long,
        default_value = "5",
        help = "Retain at most INT secondary alignments"
    )]
    pub best_n: i32,

    // Alignment scoring parameters
    #[clap(short = 'A', long, default_value = "2", help = "Matching score")]
    pub a: i32,

    #[clap(
        short = 'B',
        long,
        default_value = "4",
        help = "Mismatch penalty (larger value for lower divergence)"
    )]
    pub b: i32,

    #[clap(short = 'O', long, value_parser = parse_integer_tuple, default_value = "4,24", help = "Gap open penalties. Format: INT,INT")]
    pub gap_open: (i32, i32),

    #[clap(short = 'E', long, value_parser = parse_integer_tuple, default_value = "2,1", help = "Gap extension penalties. Format: INT,INT")]
    pub gap_ext: (i32, i32),

    #[clap(short = 'z', long, value_parser = parse_integer_tuple, default_value = "400,200", help = "Z-drop score and inversion Z-drop score")]
    pub zdrop: (i32, i32),
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
