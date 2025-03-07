use std::io::Write;

use anyhow::{bail, Result};
use minimap2::{Aligner, Built, IdxOpt, MapOpt};

use crate::cli::{IndexOptions, MappingOptions};

pub fn build_index(
    path: &str,
    map_options: MappingOptions,
    idx_options: IndexOptions,
    n_threads: usize,
    log_options: bool,
) -> Result<Aligner<Built>> {
    eprintln!("Building index...");
    let start = std::time::Instant::now();
    let aligner = Aligner::builder()
        .preset(idx_options.preset.into())
        .with_index_threads(n_threads)
        .with_index(path, None);
    let duration = start.elapsed();
    eprintln!("Index built in {:?}", duration);

    match aligner {
        Ok(mut aligner) => {
            update_map_options(&mut aligner, map_options);
            update_index_options(&mut aligner, idx_options);

            if log_options {
                pprint_index(&mut std::io::stderr(), aligner.idxopt)?;
                pprint_map(&mut std::io::stderr(), aligner.mapopt)?;
            }

            Ok(aligner)
        }
        Err(err) => bail!("Error building index: {}", err),
    }
}

fn update_map_options(aligner: &mut Aligner<Built>, map_options: MappingOptions) {
    if let Some(mask_level) = map_options.mask_level {
        aligner.mapopt.mask_level = mask_level;
    }
    if let Some(max_gap) = map_options.max_gap {
        aligner.mapopt.max_gap = max_gap;
    }
    if let Some(max_gap_ref) = map_options.max_gap_ref {
        aligner.mapopt.max_gap_ref = max_gap_ref;
    }
    if let Some(max_frag_len) = map_options.max_frag_len {
        aligner.mapopt.max_frag_len = max_frag_len;
    }
    if let Some(bandwidth) = map_options.bandwidth {
        aligner.mapopt.bw = bandwidth.0;
        aligner.mapopt.bw_long = bandwidth.1;
    }
    if let Some(min_cnt) = map_options.min_cnt {
        aligner.mapopt.min_cnt = min_cnt;
    }
    if let Some(min_chain_score) = map_options.min_chain_score {
        aligner.mapopt.min_chain_score = min_chain_score;
    }
    if let Some(pri_ratio) = map_options.pri_ratio {
        aligner.mapopt.pri_ratio = pri_ratio;
    }
    if let Some(best_n) = map_options.best_n {
        aligner.mapopt.best_n = best_n;
    }
    if let Some(a) = map_options.a {
        aligner.mapopt.a = a;
    }
    if let Some(b) = map_options.b {
        aligner.mapopt.b = b;
    }
    if let Some(gap_open) = map_options.gap_open {
        aligner.mapopt.q = gap_open.0;
        aligner.mapopt.q2 = gap_open.1;
    }
    if let Some(gap_ext) = map_options.gap_ext {
        aligner.mapopt.e = gap_ext.0;
        aligner.mapopt.e2 = gap_ext.1;
    }
    if let Some(zdrop) = map_options.zdrop {
        aligner.mapopt.zdrop = zdrop.0;
        aligner.mapopt.zdrop_inv = zdrop.1;
    }
    if let Some(splice_mode) = map_options.splice_mode {
        splice_mode.update_mapopt(&mut aligner.mapopt);
    }
}

fn update_index_options(aligner: &mut Aligner<Built>, idx_options: IndexOptions) {
    if let Some(k) = idx_options.kmer_size {
        aligner.idxopt.k = k;
    }
    if let Some(w) = idx_options.window_size {
        aligner.idxopt.w = w;
    }
}

fn pprint_index<W: Write>(writer: &mut W, opt: IdxOpt) -> Result<()> {
    writeln!(writer, "== Index Options ==")?;
    writeln!(writer, "  k: {}", opt.k)?;
    writeln!(writer, "  w: {}", opt.w)?;
    writeln!(writer, "  flag: {}", opt.flag)?;
    writeln!(writer, "  bucket_bits: {}", opt.bucket_bits)?;
    writeln!(writer, "  mini_batch_size: {}", opt.mini_batch_size)?;
    writeln!(writer, "  batch_size: {}", opt.batch_size)?;
    Ok(())
}

fn pprint_map<W: Write>(writer: &mut W, opt: MapOpt) -> Result<()> {
    writeln!(writer, "== Mapping Options ==")?;
    writeln!(writer, "  flag: {}", opt.flag)?;
    writeln!(writer, "  seed: {}", opt.seed)?;
    writeln!(writer, "  sdust_thres: {}", opt.sdust_thres)?;
    writeln!(writer, "  max_qlen: {}", opt.max_qlen)?;
    writeln!(writer, "  bw: {}", opt.bw)?;
    writeln!(writer, "  bw_long: {}", opt.bw_long)?;
    writeln!(writer, "  max_gap: {}", opt.max_gap)?;
    writeln!(writer, "  max_gap_ref: {}", opt.max_gap_ref)?;
    writeln!(writer, "  max_frag_len: {}", opt.max_frag_len)?;
    writeln!(writer, "  max_chain_skip: {}", opt.max_chain_skip)?;
    writeln!(writer, "  max_chain_iter: {}", opt.max_chain_iter)?;
    writeln!(writer, "  min_cnt: {}", opt.min_cnt)?;
    writeln!(writer, "  min_chain_score: {}", opt.min_chain_score)?;
    writeln!(writer, "  chain_gap_scale: {}", opt.chain_gap_scale)?;
    writeln!(writer, "  chain_skip_scale: {}", opt.chain_skip_scale)?;
    writeln!(writer, "  rmq_size_cap: {}", opt.rmq_size_cap)?;
    writeln!(writer, "  rmq_inner_dist: {}", opt.rmq_inner_dist)?;
    writeln!(writer, "  rmq_rescue_size: {}", opt.rmq_rescue_size)?;
    writeln!(writer, "  rmq_rescue_ratio: {}", opt.rmq_rescue_ratio)?;
    writeln!(writer, "  mask_level: {}", opt.mask_level)?;
    writeln!(writer, "  mask_len: {}", opt.mask_len)?;
    writeln!(writer, "  pri_ratio: {}", opt.pri_ratio)?;
    writeln!(writer, "  best_n: {}", opt.best_n)?;
    writeln!(writer, "  alt_drop: {}", opt.alt_drop)?;
    writeln!(writer, "  a: {}", opt.a)?;
    writeln!(writer, "  b: {}", opt.b)?;
    writeln!(writer, "  q: {}", opt.q)?;
    writeln!(writer, "  e: {}", opt.e)?;
    writeln!(writer, "  q2: {}", opt.q2)?;
    writeln!(writer, "  e2: {}", opt.e2)?;
    writeln!(writer, "  transition: {}", opt.transition)?;
    writeln!(writer, "  sc_ambi: {}", opt.sc_ambi)?;
    writeln!(writer, "  noncan: {}", opt.noncan)?;
    writeln!(writer, "  junc_bonus: {}", opt.junc_bonus)?;
    writeln!(writer, "  zdrop: {}", opt.zdrop)?;
    writeln!(writer, "  zdrop_inv: {}", opt.zdrop_inv)?;
    writeln!(writer, "  end_bonus: {}", opt.end_bonus)?;
    writeln!(writer, "  min_dp_max: {}", opt.min_dp_max)?;
    writeln!(writer, "  min_ksw_len: {}", opt.min_ksw_len)?;
    writeln!(writer, "  anchor_ext_len: {}", opt.anchor_ext_len)?;
    writeln!(writer, "  anchor_ext_shift: {}", opt.anchor_ext_shift)?;
    writeln!(writer, "  max_clip_ratio: {}", opt.max_clip_ratio)?;
    writeln!(writer, "  rank_min_len: {}", opt.rank_min_len)?;
    writeln!(writer, "  rank_frac: {}", opt.rank_frac)?;
    writeln!(writer, "  pe_ori: {}", opt.pe_ori)?;
    writeln!(writer, "  pe_bonus: {}", opt.pe_bonus)?;
    writeln!(writer, "  mid_occ_frac: {}", opt.mid_occ_frac)?;
    writeln!(writer, "  q_occ_frac: {}", opt.q_occ_frac)?;
    writeln!(writer, "  min_mid_occ: {}", opt.min_mid_occ)?;
    writeln!(writer, "  max_mid_occ: {}", opt.max_mid_occ)?;
    writeln!(writer, "  mid_occ: {}", opt.mid_occ)?;
    writeln!(writer, "  max_occ: {}", opt.max_occ)?;
    writeln!(writer, "  max_max_occ: {}", opt.max_max_occ)?;
    writeln!(writer, "  occ_dist: {}", opt.occ_dist)?;
    writeln!(writer, "  mini_batch_size: {}", opt.mini_batch_size)?;
    writeln!(writer, "  max_sw_mat: {}", opt.max_sw_mat)?;
    writeln!(writer, "  cap_kalloc: {}", opt.cap_kalloc)?;

    // For the pointer field, we need to handle it carefully
    // This is a C-style string pointer, so we should print it safely
    if !opt.split_prefix.is_null() {
        // Note: This is unsafe and assumes the string is valid UTF-8
        // In a real implementation, you might want more robust handling
        unsafe {
            let c_str = std::ffi::CStr::from_ptr(opt.split_prefix);
            if let Ok(str_slice) = c_str.to_str() {
                writeln!(writer, "  split_prefix: {}", str_slice)?;
            } else {
                writeln!(writer, "  split_prefix: <invalid UTF-8>")?;
            }
        }
    } else {
        writeln!(writer, "  split_prefix: <null>")?;
    }

    Ok(())
}
