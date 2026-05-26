use std::hash::Hash;

use apache_avro::{Reader, from_value, types::Value};
use serde::{Deserialize};

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
pub struct DataFileRecord {
    pub content: i32,
    pub file_path: String,
    pub file_format: String,
    pub partition_data: Option<String>,
    pub record_count: i64,
    pub file_size_in_bytes: i64,
    //pub column_sizes: Option<Vec<ColumnSize>>,
}

impl ManifestFile {
    pub fn from_file(path: &str) -> ManifestFile {
        let file = std::fs::File::open(path)
            .expect("Failed to open manifest file");
        let reader = Reader::new(file)
            .expect("Failed to create Avro reader");
        let record = reader.into_iter().next()
            .expect("No records found in manifest file")
            .expect("Failed to read record from Avro file");
        from_value(&record)
            .expect("Failed to parse record")
    }

}  