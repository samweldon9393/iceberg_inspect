use iceberg_inspect::parse;

use comfy_table::Table;


pub fn list_snapshots(table: &str) {
    println!("Executing snapshots command with argument: {}", table);
    
    let metadata = parse::metadata::TableMetadata::from_file(table)
        .expect("Failed to read metadata");
    println!("Table UUID: {:?}", metadata.table_uuid);
    
    let snapshots = metadata.snapshots.unwrap_or_default();
    let mut table = comfy_table::Table::new();
    table.load_preset(comfy_table::presets::NOTHING);
    table.set_header(vec!["Snapshot ID", "Sequence Number", "Parent Snapshot ID", "Timestamp (ms)"]);

    for snapshot in snapshots {
        table.add_row(vec![
            format!("{:?}", snapshot.snapshot_id.unwrap_or_default()),
            format!("{:?}", snapshot.sequence_number.unwrap_or_default()),
            format!("{:?}", snapshot.parent_snapshot_id.unwrap_or_default()),
            format!("{:?}", snapshot.timestamp_ms.unwrap_or_default()),
        ]);
    }

    println!("{}", table);
}