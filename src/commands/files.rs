use anyhow::Result as AnyResult;

use iceberg_inspect::parse::{self, manifest_list};
use comfy_table::*;

pub async fn list_files(table: &str, region: Option<&str>, snapshot_id: Option<&str>) -> AnyResult<()> {
    let files = parse::get_datafiles(table, region, snapshot_id).await?;

    let mut table = Table::new();
    table.set_header(vec!["Path", "Format", "Record Count", "File Size"])
                .load_preset(comfy_table::presets::UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic);

    for file in files {
        table.add_row(vec![
            file.file_path.to_string(),
            file.file_format.to_string(),
            file.record_count.to_string(),
            file.file_size_in_bytes.to_string(),
        ]);
    }

    println!("{}", table);
    anyhow::Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_files() {
        // This test assumes you have a valid Iceberg table metadata file at the specified path.
        let result = list_files("s3://iceberg-sandbox/mydb/mytable/metadata/00002-a0902076-c7b9-4be2-83e1-5987041a6779.metadata.json", Some("us-east-2"), None);
        assert!(result.await.is_ok());
    }
}