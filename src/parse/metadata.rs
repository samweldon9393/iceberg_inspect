use serde::{Deserialize, Serialize};
use std::{collections::HashMap};
use anyhow::Result as AnyResult;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TableMetadata {
    #[serde(rename = "format-version")]
    pub format_version: Option<u8>,
    pub location: Option<String>,
    #[serde(rename = "table-uuid")]
    pub table_uuid: Option<String>,
    #[serde(rename = "current-snapshot-id")]
    pub current_snapshot_id: Option<i64>,
    pub snapshots: Option<Vec<Snapshot>>,
    pub schemas: Option<Vec<TableSchema>>,
    #[serde(rename = "current-schema-id")]
    pub current_schema_id: Option<i32>,

    #[serde(rename = "last-updated-ms")]
    pub last_updated_ms: Option<i64>,
    #[serde(rename = "last-column-id")]
    pub last_column_id: Option<i32>,
    #[serde(rename = "default-spec-id")]
    pub default_spec_id: Option<i32>,
    #[serde(rename = "last-partition-id")]
    pub last_partition_id: Option<i32>,
    #[serde(rename = "default-sort-order-id")]
    pub default_sort_order_id: Option<i32>,
    #[serde(rename = "last-sequence-number")]
    pub last_sequence_number: Option<i64>,

    /*
    Complex types that may not be needed
    #[serde(rename = "partition-specs")]
    partition_specs: Option<Vec<HashMap<String, String>>>,
    snapshot_log: Option<??>,
    metadata_log: Option<??>,
    sort_orders: Option<Vec<??>,
    partition_statistics: Option<??>,
    statistics: Option<??>,
    refs: Option<??>,
    properties: Option<??>,
    */
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableSchema {
    #[serde(rename = "type")]
    pub _type: Option<String>,  // e.g., "struct"
    pub fields: Option<Vec<SchemaField>>,  // List of fields with their types and other metadata
    #[serde(rename = "identifier-field-ids")]
    pub identifier_field_ids: Option<Vec<i32>>,
    #[serde(rename = "schema-id")]
    pub schema_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaField {
    pub id: Option<i32>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub _type: Option<String>,  // e.g., "string", "integer", "struct", "list", "map", etc.
    pub required: Option<bool>,
    pub doc: Option<String>,
    pub default: Option<serde_json::Value>,  // Default value for the field, if any
    // For complex types like struct, list, map, we might need additional metadata
    // For example, for struct we might have a list of nested fields, for list we might have an element type, etc.
    // This can be represented as a recursive structure or as a more flexible JSON value depending on the complexity of the types we want to support.
    // For simplicity, we can start with just the basic types and add support for complex types later as needed.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    #[serde(rename = "snapshot-id")]
    pub snapshot_id: Option<i64>,
    #[serde(rename = "sequence-number")]
    pub sequence_number: Option<i64>,
    #[serde(rename = "schema-id")]
    pub schema_id: Option<i32>,
    #[serde(rename = "parent-snapshot-id")]
    pub parent_snapshot_id: Option<i64>,
    #[serde(rename = "timestamp-ms")]
    pub timestamp_ms: Option<i64>,
    #[serde(rename = "manifest-list")]
    pub manifest_list: Option<String>,  // path to the Avro manifest list file
    pub summary: Option<HashMap<String, String>>,
}


impl TableMetadata {
    #[allow(dead_code)]
    pub fn from_file(path: &str) -> AnyResult<Self> {
        let json_str = std::fs::read_to_string(path)?;
        let metadata: TableMetadata = serde_json::from_str(&json_str)?;
        Ok(metadata)
    }

    #[allow(dead_code)]
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }
    
    #[allow(dead_code)]
    pub fn get_current_snapshot(&self) -> Option<&Snapshot> {
        if let Some(current_snapshot_id) = self.current_snapshot_id {
            if let Some(snapshots) = &self.snapshots {
                return snapshots.iter().find(|s| s.snapshot_id == Some(current_snapshot_id));
            }
        }
        None
    }

    #[allow(dead_code)]
    pub fn get_snapshot(&self, snapshot_id: Option<&str>) -> Option<&Snapshot> {
        if let Some(sid) = snapshot_id {
            if let Some(snapshots) = &self.snapshots {
                return snapshots
                    .iter()
                    .find(|s| s.snapshot_id == Some(sid.parse::<i64>().unwrap_or_default()));
            }
            else {
                return None;
            }
        } else {
            return self.get_current_snapshot();
        }
    }
}

impl Snapshot {
    #[allow(dead_code)]
    pub fn get_manifest_list_path(&self) -> Option<String> {
        if let Some(manifest_list_path) = &self.manifest_list {
            Some(manifest_list_path.clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_file() {
        // TODO change to use S3 once that's added later
        let metadata = TableMetadata::from_file("./taxis/metadata/00003-3b45d19f-94fb-4ea3-8d77-d769539ba79c.metadata.json").unwrap();
        assert_eq!(metadata.table_uuid, Some("3aaeb9f7-207f-4d66-91d5-b46c433d359b".into()));
    }
}