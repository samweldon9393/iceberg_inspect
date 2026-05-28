use anyhow::Result as AnyResult;

use iceberg_inspect::parse::{self, manifest_list};
use comfy_table::*;

pub async fn list_files(table: &str, snapshot_id: Option<&str>) -> AnyResult<()> {
    let files = parse::get_datafiles(table, snapshot_id).await?;

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
        let result = list_files("./taxis/metadata/00003-3b45d19f-94fb-4ea3-8d77-d769539ba79c.metadata.json", None);
        assert!(result.await.is_ok());
    }
}