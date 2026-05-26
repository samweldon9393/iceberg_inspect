use apache_avro::Reader;
use serde::Deserialize;

#[derive(Default, Debug, Clone)]
pub struct ManifestList {
    pub records: Vec<apache_avro::types::Value>,
}

impl ManifestList {
    pub fn default() -> Self {
        ManifestList {
            records: Vec::new(),
        }
    }
    pub fn add_record(&mut self, record: apache_avro::types::Value) {
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
        manifest_list.add_record(record);
    }

    manifest_list
}