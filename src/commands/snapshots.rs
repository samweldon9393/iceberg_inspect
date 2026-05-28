use iceberg_inspect::parse;
use anyhow::Result as AnyResult;
use std::collections::HashMap;

use comfy_table::*;

// TODO - Add ability to filter snapshots by timestamp or sequence number
pub async fn list_snapshots(table: &str, region: Option<&str>) -> AnyResult<()> {
    let metadata = parse::metadata::TableMetadata::from_file(table, region).await?;
    
    let snapshots = metadata.snapshots.unwrap_or_default();
    let mut table = comfy_table::Table::new();
    table
        .load_preset(comfy_table::presets::UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Sequence Number", "Snapshot ID", "Parent Snapshot ID", "Timestamp (ms)", "Added Records"]);

    for snapshot in snapshots {
        table.add_row(vec![
            format!("{:?}", snapshot.sequence_number.unwrap_or_default()),
            format!("{:?}", snapshot.snapshot_id.unwrap_or_default()),
            format!("{:?}", snapshot.parent_snapshot_id.unwrap_or_default()),
            format!("{:?}", snapshot.timestamp_ms.unwrap_or_default()),
            format!("{:?}", snapshot.summary
                                .as_ref()
                                .unwrap_or(&HashMap::new())
                                .get("added-records")
                                .unwrap_or(&String::from("0"))
                                .parse::<i32>()
                                .unwrap_or(0))
        ]);
    }

    println!("{}", table);
    anyhow::Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_snapshots() {
        // TODO change to use S3 once that's added later
        let result = list_snapshots("s3://iceberg-sandbox/mydb/mytable/metadata/00002-a0902076-c7b9-4be2-83e1-5987041a6779.metadata.json", Some("us-east-2"));
        assert!(result.await.is_ok());
    }
}