use apache_avro::{Reader, from_value};
use serde::{Deserialize};

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
    pub fn default() -> Self {
        ManifestList {
            records: Vec::new(),
        }
    }
    pub fn add_record(&mut self, record: ManifestListRecord) {
        self.records.push(record);
    }

    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let reader = Reader::new(file)?;
        let mut manifest_list = ManifestList::default();

        for record in reader {
            let record = record.expect("Failed to read record from Avro file");
            if let Ok(manifest_record) = from_value::<ManifestListRecord>(&record) {
                manifest_list.add_record(manifest_record);
            }
        }

        Ok(manifest_list)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_file() {
        let manifest_list = ManifestList::from_file("./taxis/metadata/snap-7143047217624574150-0-d6877f7e-bceb-480d-a2a0-d63fbe20045f.avro").unwrap();
        assert_eq!(manifest_list.records[0].manifest_length, 6688);
    }
}
