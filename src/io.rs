use std::fs::File;
use std::io::{Read, Write};

use anyhow::Result;

pub fn transparent_reader(input: &str) -> Result<Box<dyn Read + Send>> {
    let (stream, _comp) = niffler::send::from_path(input)?;
    Ok(stream)
}

pub fn transparent_writer(output: Option<&str>) -> Result<Box<dyn Write + Send>> {
    if let Some(path) = output {
        let stream = File::create(path)?;
        Ok(Box::new(stream))
    } else {
        Ok(Box::new(std::io::stderr()))
    }
}
