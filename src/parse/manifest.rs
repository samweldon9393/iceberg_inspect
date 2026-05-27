use super::data_file::DataFileRecord;
use std::hash::Hash;

use apache_avro::{Reader, from_value, types::Value};
use serde::{Deserialize};

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct ManifestFile {
    pub status: i32,
    pub snapshot_id: i64,
    pub sequence_number: Option<i64>,
    pub file_sequence_number: Option<i64>,
    pub data_file: DataFileRecord,

    pub key_metadata: Option<String>,
    pub split_offsets: Option<String>,
    pub equality_ids: Option<String>,
    pub sort_order_id: Option<i64>,
}

impl ManifestFile {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let reader = Reader::new(file)?;
        let record = reader.into_iter().next().unwrap()?;
        from_value(&record)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_file() {
        let manifest = ManifestFile::from_file("./taxis/metadata/d6877f7e-bceb-480d-a2a0-d63fbe20045f-m0.avro").unwrap();
        assert_eq!(manifest.snapshot_id, 7143047217624574150);
    }
}