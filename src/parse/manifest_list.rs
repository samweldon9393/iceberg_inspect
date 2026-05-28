use std::any::Any;

use apache_avro::{Reader, from_value};
use serde::{Deserialize};
use anyhow::Result as AnyResult;

use crate::parse;

#[derive(Default, Debug, Clone)]
pub struct ManifestList {
    pub records: Vec<ManifestListRecord>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn default() -> Self {
        ManifestList {
            records: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add_record(&mut self, record: ManifestListRecord) {
        self.records.push(record);
    }

    #[allow(dead_code)]
    pub async fn from_file(path: &str, region: Option<&str>) -> AnyResult<Self> {
        let cursor = parse::read_bytes(path, region).await?;
        let reader = Reader::new(cursor)?;
        let mut manifest_list = ManifestList::default();

        for record in reader {
            let record = record?;
            if let Ok(manifest_record) = from_value::<ManifestListRecord>(&record) {
                manifest_list.add_record(manifest_record);
            }
        }

        anyhow::Ok(manifest_list)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_from_file() {
        let manifest_list = ManifestList::from_file("s3://iceberg-sandbox/mydb/mytable/metadata/snap-7065593814654901936-0-0d3e3d48-0988-4ed8-b3c5-71cf36d0ce61.avro", Some("us-east-2")).await.unwrap();
        assert_eq!(manifest_list.records[0].manifest_length, 10298);
    }
}
