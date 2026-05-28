use super::data_file::DataFileRecord;
use anyhow::Result as AnyResult;
use std::io::Cursor;

use apache_avro::{Reader, from_value};
use serde::{Deserialize};
use crate::parse::read_bytes;

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
    #[allow(dead_code)]
    pub async fn from_file(path: &str, region: Option<&str>) -> AnyResult<Self> {
        let cursor = read_bytes(path, region).await?;
        let reader = Reader::new(cursor)?;
        let record = reader.into_iter().next().unwrap()?;
        from_value(&record)
            .map_err(|e| anyhow::anyhow!("Failed to parse manifest file: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_from_file() {
        let manifest = ManifestFile::from_file("./taxis/metadata/d6877f7e-bceb-480d-a2a0-d63fbe20045f-m0.avro", Some("")).await.unwrap();
        assert_eq!(manifest.snapshot_id, 7143047217624574150);
    }
}