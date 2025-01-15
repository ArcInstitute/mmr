use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
    num::NonZeroI32,
    sync::Arc,
};

use anyhow::{bail, Result};
use binseq::{BinseqRecord, ParallelProcessor, RefRecord};
use minimap2::{Aligner, Built, Mapping};
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
}
impl ParallelAlignment {
    pub fn new(aligner: Aligner<Built>, output_path: Option<String>) -> Result<Self> {
        Self::initialize_output(output_path.as_ref())?;
        Ok(Self {
            aligner: Arc::new(aligner),
            dbuf: Vec::new(),
            wbuf: Vec::new(),
            io_lock: Arc::new(Mutex::new(())),
            output_path,
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
    fn decode_record(&mut self, record: RefRecord) -> Result<()> {
        self.dbuf.clear();
        record.decode(&mut self.dbuf)?;
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
}
impl ParallelProcessor for ParallelAlignment {
    fn process_record(&mut self, record: RefRecord) -> Result<()> {
        self.decode_record(record)?;
        let mapping = match self.aligner.map(&self.dbuf, false, false, None, None, None) {
            Ok(mapping) => mapping,
            Err(err) => bail!("Error mapping record: {}", err),
        };

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

    fn on_batch_complete(&mut self) -> Result<()> {
        self.write_record_set()?;
        Ok(())
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
