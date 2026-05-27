use anyhow::Result as AnyResult;

use iceberg_inspect::parse::{self, manifest_list};
use comfy_table::*;

pub fn list_files(table: &str, snapshot_id: Option<&str>) -> AnyResult<()> {
    /* Start by parsing the table metadata to get the correct manifest list (snapshot) */
    let metadata = parse::metadata::TableMetadata::from_file(table)?;
    let snapshots = metadata.snapshots.unwrap_or_default();
    let snapshot = if let Some(sid) = snapshot_id {
        snapshots.into_iter().find(|s| s.snapshot_id.as_ref().map(|id| id.to_string()) == Some(sid.to_string()))
    } else {
        snapshots.into_iter().max_by_key(|s| s.timestamp_ms.unwrap_or_default())
    };
    let manifest_list_path = if let Some(snapshot_unwrapped) = snapshot {
        snapshot_unwrapped.get_manifest_list_path()
    } else {
        anyhow::bail!("No matching snapshot found");
    };
    
    /* Then parse the manifest list to get the list of manifest files */
    let manifest_list = if let Some(path) = manifest_list_path {
        manifest_list::ManifestList::from_file(&path)?
    } else {
        anyhow::bail!("No manifest list path found for snapshot");
    };
    let manifest_file_paths = manifest_list.records
                                        .into_iter()
                                        .map(|record| record.manifest_path)
                                        .collect::<Vec<String>>();

    
    /* Finally, extract the list of data files from parsed manifest files */
    let mut files: Vec<parse::data_file::DataFileRecord> = Vec::new();
    for manifest_path in manifest_file_paths {
        let manifest = parse::manifest::ManifestFile::from_file(&manifest_path)?;
        files.push(manifest.data_file);
    }
    

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

    #[test]
    fn test_list_files() {
        // This test assumes you have a valid Iceberg table metadata file at the specified path.
        let result = list_files("./taxis/metadata/00003-3b45d19f-94fb-4ea3-8d77-d769539ba79c.metadata.json", None);
        assert!(result.is_ok());
    }
}