extern crate tempfile;

use std::io::Write;
use tempfile::NamedTempFile;

pub struct Tempfile {
    pub file: NamedTempFile,
    pub filepath: String,
}

impl Tempfile {
    pub fn create(content: &str) -> Self {
        let mut file = NamedTempFile::new().expect("Failed to create temporary file");
        writeln!(file, "{}", content).expect("Failed to write to temporary file");

        let filepath = file
            .path()
            .to_str()
            .expect("Failed to convert temp file path to string");

        let filepath = String::from(filepath);

        Self { file, filepath }
    }
}
