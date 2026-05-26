use arrow::datatypes::Schema;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TableMetadata {
    #[serde(rename = "format-version")]
    format_version: Option<u8>,
    location: Option<String>,
    #[serde(rename = "table-uuid")]
    table_uuid: Option<String>,
    #[serde(rename = "current-snapshot-id")]
    current_snapshot_id: Option<i64>,
    snapshots: Option<Vec<Snapshot>>,
    schemas: Option<Vec<TableSchema>>,
    #[serde(rename = "current-schema-id")]
    current_schema_id: Option<i32>,

    #[serde(rename = "last-updated-ms")]
    last_updated_ms: Option<i64>,
    #[serde(rename = "last-column-id")]
    last_column_id: Option<i32>,
    #[serde(rename = "default-spec-id")]
    default_spec_id: Option<i32>,
    #[serde(rename = "last-partition-id")]
    last_partition_id: Option<i32>,
    #[serde(rename = "default-sort-order-id")]
    default_sort_order_id: Option<i32>,
    #[serde(rename = "last-sequence-number")]
    last_sequence_number: Option<i64>,

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
    _type: Option<String>,  // e.g., "struct"
    fields: Option<Vec<SchemaField>>,  // List of fields with their types and other metadata
    #[serde(rename = "identifier-field-ids")]
    identifier_field_ids: Option<Vec<i32>>,
    #[serde(rename = "schema-id")]
    schema_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaField {
    id: Option<i32>,
    name: Option<String>,
    #[serde(rename = "type")]
    _type: Option<String>,  // e.g., "string", "integer", "struct", "list", "map", etc.
    required: Option<bool>,
    doc: Option<String>,
    default: Option<serde_json::Value>,  // Default value for the field, if any
    // For complex types like struct, list, map, we might need additional metadata
    // For example, for struct we might have a list of nested fields, for list we might have an element type, etc.
    // This can be represented as a recursive structure or as a more flexible JSON value depending on the complexity of the types we want to support.
    // For simplicity, we can start with just the basic types and add support for complex types later as needed.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    #[serde(rename = "snapshot-id")]
    snapshot_id: Option<i64>,
    #[serde(rename = "sequence-number")]
    sequence_number: Option<i64>,
    #[serde(rename = "schema-id")]
    schema_id: Option<i32>,
    #[serde(rename = "parent-snapshot-id")]
    parent_snapshot_id: Option<i64>,
    #[serde(rename = "timestamp-ms")]
    timestamp_ms: Option<i64>,
    #[serde(rename = "manifest-list")]
    manifest_list: Option<String>,  // path to the Avro manifest list file
    summary: Option<HashMap<String, String>>,
}

impl TableMetadata {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json_str = std::fs::read_to_string(path)?;
        let metadata: TableMetadata = serde_json::from_str(&json_str)?;
        Ok(metadata)
    }

    #[allow(dead_code)] // TODO needed?
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }
    
    pub fn get_current_snapshot(&self) -> Option<&Snapshot> {
        if let Some(current_snapshot_id) = self.current_snapshot_id {
            if let Some(snapshots) = &self.snapshots {
                return snapshots.iter().find(|s| s.snapshot_id == Some(current_snapshot_id));
            }
        }
        None
    }
}

impl Snapshot {
    pub fn get_manifest_list_path(&self) -> Option<String> {
        if let Some(manifest_list_path) = &self.manifest_list {
            Some(manifest_list_path.clone())
        } else {
            None
        }
    }
}