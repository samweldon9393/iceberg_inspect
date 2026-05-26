use arrow::datatypes::Schema;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TableMetadata {
    format_version: Option<u8>,
    location: Option<String>,
    table_uuid: Option<String>,
    current_snapshot_id: Option<i64>,
    snapshots: Option<Vec<Snapshot>>,
    schemas: Option<Vec<TableSchema>>,
    current_schema_id: Option<i32>,
    partition_specs: Option<Vec<HashMap<String, String>>>,
    /*
    last_updated_ms: i64,
    last_column_id: i32,
    default_spec_id: i32,
    last_partition_id: i32,
    properties: Option<HashMap<String, String>>,
    snapshot_log: Option<Vec<HashMap<String, String>>>,
    metadata_log: Option<Vec<HashMap<String, String>>>,
    sort_orders: Option<Vec<HashMap<String, String>>>,
    default_sort_order_id: i32,
    refs: Option<HashMap<String, String>>,
    statistics: Option<HashMap<String, String>>,
    partition_statistics: Option<HashMap<String, String>>,
    last_sequence_number: i64,
    */
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableSchema {
    _type: Option<String>,  // e.g., "struct"
    fields: Option<Vec<SchemaField>>,  // List of fields with their types and other metadata
    identifier_field_ids: Option<Vec<i32>>,
    schema_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaField {
    id: Option<i32>,
    name: Option<String>,
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
    snapshot_id: Option<i64>,
    sequence_number: Option<i64>,
    schema_id: Option<i32>,
    parent_snapshot_id: Option<i64>,
    timestamp_ms: Option<i64>,
    manifest_list: Option<String>,  // path to the Avro manifest list file
    summary: Option<HashMap<String, String>>,
}

impl TableMetadata {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json_str = std::fs::read_to_string(path)?;
        let metadata: TableMetadata = serde_json::from_str(&json_str)?;
        Ok(metadata)
    }

    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }
}