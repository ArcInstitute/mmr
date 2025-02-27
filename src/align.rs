use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
    num::NonZeroI32,
    sync::Arc,
    time::Instant,
};

use anyhow::{anyhow, Result};
use binseq::RefRecord;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use minimap2::{Aligner, Built, Mapping};
use paraseq::{fastx::Record, parallel::ProcessError};
use parking_lot::Mutex;
use serde::Serialize;

#[derive(Clone)]
pub struct ParallelAlignment {
    aligner: Arc<Aligner<Built>>,

    /// Local buffer for decoding records
    dbuf: Vec<u8>,

    /// Local write buffer for PAF records
    wbuf: Vec<u8>,

    /// Path name for the output file
    output_path: Option<String>,

    /// IO lock
    io_lock: Arc<Mutex<()>>,

    /// Number of records processed (local/global)
    local_n_processed: usize,
    global_n_processed: Arc<Mutex<usize>>,

    /// Start time
    start_time: Instant,

    /// Thread id (local)
    tid: usize,

    /// Progress bar
    pbar: Arc<Mutex<ProgressBar>>,
}
impl ParallelAlignment {
    pub fn new(aligner: Aligner<Built>, output_path: Option<String>) -> Result<Self> {
        Self::initialize_output(output_path.as_ref())?;
        let pbar = Self::initialize_pbar();
        Ok(Self {
            aligner: Arc::new(aligner),
            dbuf: Vec::new(),
            wbuf: Vec::new(),
            io_lock: Arc::new(Mutex::new(())),
            local_n_processed: 0,
            global_n_processed: Arc::new(Mutex::new(0)),
            output_path,
            start_time: Instant::now(),
            tid: 0,
            pbar: Arc::new(Mutex::new(pbar)),
        })
    }
    pub fn initialize_output(output_path: Option<&String>) -> Result<()> {
        if let Some(path) = output_path {
            File::create(path)?;
            Ok(())
        } else {
            Ok(())
        }
    }
    pub fn initialize_pbar() -> ProgressBar {
        let pbar = ProgressBar::new_spinner();
        pbar.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.cyan} [{elapsed_precise}] {msg}")
                .unwrap(),
        );
        pbar.set_draw_target(ProgressDrawTarget::stderr_with_hz(10));
        pbar
    }
    fn decode_record(&mut self, record: RefRecord) -> Result<()> {
        self.dbuf.clear();
        record.decode_s(&mut self.dbuf)?;
        Ok(())
    }
    fn reopen_handle(&self) -> Result<Box<dyn Write>> {
        if let Some(path) = &self.output_path {
            let file = OpenOptions::new().append(true).open(path)?;
            let buffer = BufWriter::new(file);
            Ok(Box::new(buffer))
        } else {
            let file = std::io::stdout();
            let buffer = BufWriter::new(file);
            Ok(Box::new(buffer))
        }
    }
    fn write_local(&mut self, mapping: Vec<Mapping>) -> Result<()> {
        let mut wtr = csv::WriterBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .from_writer(&mut self.wbuf);

        for alignment in mapping {
            let mapping: MappingNutype = alignment.into();
            wtr.serialize(mapping)?;
        }
        wtr.flush()?;
        Ok(())
    }
    fn write_record_set(&mut self) -> Result<()> {
        // Open a thread-safe stdout writer
        //
        // Drops lock when it goes out of scope
        {
            let _lock = self.io_lock.lock();
            let mut handle = self.reopen_handle()?;
            handle.write_all(&self.wbuf)?;
            handle.flush()?;
        }

        // Clear the write buffer
        self.wbuf.clear();

        Ok(())
    }
    fn calculate_throughput(&self) -> f64 {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        *self.global_n_processed.lock() as f64 / elapsed
    }
    fn update_statistics(&mut self) {
        *self.global_n_processed.lock() += self.local_n_processed;
        self.local_n_processed = 0;
    }
    fn update_pbar(&self) {
        // only update progress bar on the main thread
        if self.tid == 0 {
            let pbar = self.pbar.lock();
            let elapsed = self.start_time.elapsed().as_secs_f64();
            let throughput = self.calculate_throughput();
            let msg = format!("Elapsed: {elapsed:.2}s, Throughput: {throughput:.2} reads/s",);
            pbar.set_message(msg);
        }
    }
    pub fn finish_pbar(&self) {
        let pbar = self.pbar.lock();
        let elapsed = self.start_time.elapsed().as_secs_f64();
        let throughput = self.calculate_throughput();
        let msg = format!("Elapsed: {elapsed:.2}s, Throughput: {throughput:.2} reads/s",);
        pbar.finish_with_message(msg);
    }
    pub fn start_time(&self) -> Instant {
        self.start_time
    }
    pub fn num_records(&self) -> usize {
        *self.global_n_processed.lock()
    }
}
impl binseq::ParallelProcessor for ParallelAlignment {
    fn process_record(&mut self, record: RefRecord) -> Result<(), binseq::Error> {
        self.decode_record(record)?;
        let mapping = match self.aligner.map(&self.dbuf, false, false, None, None, None) {
            Ok(mapping) => mapping,
            Err(err) => return Err(anyhow!("Error mapping record: {}", err).into()),
        };
        self.local_n_processed += 1;
        self.write_local(mapping)?;
        Ok(())
    }

    fn on_batch_complete(&mut self) -> Result<(), binseq::Error> {
        self.write_record_set()?;
        self.update_statistics();
        self.update_pbar();
        Ok(())
    }

    fn set_tid(&mut self, tid: usize) {
        self.tid = tid;
    }
}
impl paraseq::parallel::ParallelProcessor for ParallelAlignment {
    fn process_record<Rf: Record>(&mut self, record: Rf) -> paraseq::parallel::Result<()> {
        let mapping = match self
            .aligner
            .map(record.seq(), false, false, None, None, None)
        {
            Ok(mapping) => mapping,
            Err(err) => {
                return Err(ProcessError::from(anyhow!("Error mapping record: {}", err)));
            }
        };
        self.local_n_processed += 1;
        self.write_local(mapping)?;
        Ok(())
    }

    fn on_batch_complete(&mut self) -> paraseq::parallel::Result<()> {
        self.write_record_set()?;
        self.update_statistics();
        self.update_pbar();
        Ok(())
    }

    fn set_thread_id(&mut self, thread_id: usize) {
        self.tid = thread_id;
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MappingNutype {
    pub query_name: &'static str,
    pub query_len: Option<NonZeroI32>,
    pub query_start: i32,
    pub query_end: i32,
    pub strand: u8,
    pub target_name: Option<Arc<String>>,
    pub target_len: i32,
    pub target_start: i32,
    pub target_end: i32,
    pub match_len: i32,
    pub block_len: i32,
    pub mapq: u32,
    pub is_primary: bool,
    pub is_supplementary: bool,
    pub alignment: &'static str,
}
impl From<Mapping> for MappingNutype {
    fn from(mapping: Mapping) -> Self {
        MappingNutype {
            query_name: "*",
            query_len: mapping.query_len,
            query_start: mapping.query_start,
            query_end: mapping.query_end,
            strand: mapping.strand as u8,
            target_name: mapping.target_name,
            target_len: mapping.target_len,
            target_start: mapping.target_start,
            target_end: mapping.target_end,
            match_len: mapping.match_len,
            block_len: mapping.block_len,
            mapq: mapping.mapq,
            is_primary: mapping.is_primary,
            is_supplementary: mapping.is_supplementary,
            alignment: "*",
        }
    }
}
