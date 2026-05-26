use apache_avro::{Reader, from_value};
use serde::{Deserialize};

#[derive(Default, Debug, Clone)]
pub struct ManifestList {
    pub records: Vec<ManifestListRecord>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ManifestListRecord {
    pub manifest_path: String,
    pub manifest_length: i64,
    pub partition_spec_id: i32,
    pub content: i8,
    pub sequence_number: i64,
    pub added_snapshot_id: i64,
    pub deleted_snapshot_id: Option<i64>,
}

impl ManifestList {
    pub fn default() -> Self {
        ManifestList {
            records: Vec::new(),
        }
    }
    pub fn add_record(&mut self, record: ManifestListRecord) {
        self.records.push(record);
    }
    pub fn from_file(path: &str) -> Self {
        parse_manifest_list(path)
    }
}

pub fn parse_manifest_list(path: &str) -> ManifestList {
    let file = std::fs::File::open(path).expect("Failed to open manifest list file");
    let reader = Reader::new(file).expect("Failed to create Avro reader");
    let mut manifest_list = ManifestList::default();

    for record in reader {
        let record = record.expect("Failed to read record from Avro file");
        let manifest_record: ManifestListRecord = from_value(&record)
            .expect("Failed to deserialize Avro record into ManifestListRecord");
        manifest_list.add_record(manifest_record);
    }

    manifest_list
}