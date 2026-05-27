use iceberg_inspect::parse;
use anyhow::Result;

use comfy_table::*;

// TODO - Add ability to filter snapshots by timestamp or sequence number
pub fn list_snapshots(table: &str) -> anyhow::Result<()> {
    let metadata = parse::metadata::TableMetadata::from_file(table)?;
    println!("Table UUID: {:?}", metadata.table_uuid.unwrap_or_default());
    
    let snapshots = metadata.snapshots.unwrap_or_default();
    let mut table = comfy_table::Table::new();
    table
        .load_preset(comfy_table::presets::UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Snapshot ID", "Sequence Number", "Parent Snapshot ID", "Timestamp (ms)"]);

    for snapshot in snapshots {
        table.add_row(vec![
            format!("{:?}", snapshot.snapshot_id.unwrap_or_default()),
            format!("{:?}", snapshot.sequence_number.unwrap_or_default()),
            format!("{:?}", snapshot.parent_snapshot_id.unwrap_or_default()),
            format!("{:?}", snapshot.timestamp_ms.unwrap_or_default()),
        ]);
    }

    println!("{}", table);
    anyhow::Ok(())
}