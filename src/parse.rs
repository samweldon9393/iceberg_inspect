/**
 * parse.rs
 * 
 * This module contains the logic for parsing Iceberg metadata files,
 * including manifest lists, manifest files, and data files.
 */
use anyhow::Result as AnyResult;
use std::io::Cursor;

pub mod data_file;
pub mod manifest_list;
pub mod manifest;
pub mod metadata;

pub use metadata::TableMetadata;
pub use manifest_list::ManifestList;
pub use manifest::ManifestFile;
pub use data_file::DataFileRecord;

/* Do a full parse, from a table path and snapshot ID to a list of data files */
pub fn get_datafiles(table_path: &str, snapshot_id: Option<&str>) -> AnyResult<Vec<DataFileRecord>> {
    /* Start by parsing the table metadata to get the correct manifest list (snapshot) */
    let metadata = metadata::TableMetadata::from_file(table_path)?;
    let snapshot = metadata.get_snapshot(snapshot_id);
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
    let mut files: Vec<data_file::DataFileRecord> = Vec::new();
    for manifest_path in manifest_file_paths {
        let manifest = manifest::ManifestFile::from_file(&manifest_path)?;
        files.push(manifest.data_file);
    }
    anyhow::Ok(files)
}

pub fn read_bytes(path: &str) -> AnyResult<Cursor<Vec<u8>>> {
    if path.starts_with("s3://") {
        /* TODO Implement S3 
        let (bucket, key) = parse_s3_path(path);
        let store = AmazonS3Builder::from_env().with_bucket_name(bucket).build()?;
        Ok(store.get(&key.into()).await?.bytes().await?)
        */
        let bytes = std::fs::read(path)?;
        anyhow::Ok(std::io::Cursor::new(bytes))
    } else {
        let bytes = std::fs::read(path)?;
        anyhow::Ok(std::io::Cursor::new(bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_datafiles() {
        let table_path = "./taxis/metadata/00003-3b45d19f-94fb-4ea3-8d77-d769539ba79c.metadata.json";
        assert_eq!(get_datafiles(table_path, None).unwrap().len(), 2);
    }
}