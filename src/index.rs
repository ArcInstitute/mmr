use anyhow::{bail, Result};
use minimap2::{Aligner, Built};

use crate::cli::{IndexOptions, MappingOptions};

pub fn build_index(
    path: &str,
    map_options: MappingOptions,
    idx_options: IndexOptions,
    n_threads: usize,
) -> Result<Aligner<Built>> {
    eprintln!("Building index...");
    let start = std::time::Instant::now();
    let aligner = Aligner::builder()
        .preset(minimap2::Preset::Sr)
        .with_index_threads(n_threads)
        .with_index(path, None);
    let duration = start.elapsed();
    eprintln!("Index built in {:?}", duration);

    match aligner {
        Ok(mut aligner) => {
            update_map_options(&mut aligner, map_options);
            update_index_options(&mut aligner, idx_options);
            Ok(aligner)
        }
        Err(err) => bail!("Error building index: {}", err),
    }
}

fn update_map_options(aligner: &mut Aligner<Built>, map_options: MappingOptions) {
    aligner.mapopt.mask_level = map_options.mask_level;
    aligner.mapopt.max_gap = map_options.max_gap;
    aligner.mapopt.max_gap_ref = map_options.max_gap_ref;
    aligner.mapopt.max_frag_len = map_options.max_frag_len;
    aligner.mapopt.bw = map_options.bandwidth.0;
    aligner.mapopt.bw_long = map_options.bandwidth.1;
    aligner.mapopt.min_cnt = map_options.min_cnt;
    aligner.mapopt.min_chain_score = map_options.min_chain_score;
    aligner.mapopt.pri_ratio = map_options.pri_ratio;
    aligner.mapopt.best_n = map_options.best_n;
    aligner.mapopt.a = map_options.a;
    aligner.mapopt.b = map_options.b;
    aligner.mapopt.q = map_options.gap_open.0;
    aligner.mapopt.q2 = map_options.gap_open.1;
    aligner.mapopt.e = map_options.gap_ext.0;
    aligner.mapopt.e2 = map_options.gap_ext.1;
    aligner.mapopt.zdrop = map_options.zdrop.0;
    aligner.mapopt.zdrop_inv = map_options.zdrop.1;
}

fn update_index_options(aligner: &mut Aligner<Built>, idx_options: IndexOptions) {
    aligner.idxopt.k = idx_options.kmer_size;
    aligner.idxopt.w = idx_options.window_size;
}
