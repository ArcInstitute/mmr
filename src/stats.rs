use std::time::Instant;

use serde::Serialize;

#[derive(Serialize)]
pub struct Runtime {
    #[serde(rename = "elapsed_total_sec")]
    pub e_total: f64,
    #[serde(rename = "elapsed_init_sec")]
    pub e_init: f64,
    #[serde(rename = "elapsed_map_sec")]
    pub e_map: f64,
    #[serde(rename = "total_records")]
    pub n_records: usize,
    #[serde(rename = "throughput_records_per_sec")]
    pub throughput: f64,
}
impl Runtime {
    pub fn new(t_init: Instant, t_map: Instant, n_records: usize) -> Self {
        let e_total = t_init.elapsed().as_secs_f64();
        let e_init = (t_map - t_init).as_secs_f64();
        let e_map = t_map.elapsed().as_secs_f64();
        let throughput = n_records as f64 / e_map;
        Self {
            e_total,
            e_init,
            e_map,
            n_records,
            throughput,
        }
    }
}
