/**
 * parse.rs
 * 
 * This module contains the logic for parsing Iceberg metadata files,
 * including manifest lists, manifest files, and data files.
 */
use anyhow::Result as AnyResult;
use std::io::Cursor;
use crate::s3;
use object_store::{ObjectStoreExt, aws::AmazonS3Builder};

pub mod data_file;
pub mod manifest_list;
pub mod manifest;
pub mod metadata;

#[allow(dead_code)]
pub use metadata::TableMetadata;
pub use manifest_list::ManifestList;
pub use manifest::ManifestFile;
pub use data_file::DataFileRecord;

/* Do a full parse, from a table path and snapshot ID to a list of data files */
#[allow(dead_code)]
pub async fn get_datafiles(table_path: &str, region: Option<&str>, snapshot_id: Option<&str>) -> AnyResult<Vec<DataFileRecord>> {
    /* Start by parsing the table metadata to get the correct manifest list (snapshot) */
    let metadata = metadata::TableMetadata::from_file(table_path, region).await?;
    let snapshot = metadata.get_snapshot(snapshot_id);
    let manifest_list_path = if let Some(snapshot_unwrapped) = snapshot {
        snapshot_unwrapped.get_manifest_list_path()
    } else {
        anyhow::bail!("No matching snapshot found");
    };
    
    /* Then parse the manifest list to get the list of manifest files */
    let manifest_list = if let Some(path) = manifest_list_path {
        manifest_list::ManifestList::from_file(&path, region).await?
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
        let manifest = manifest::ManifestFile::from_file(&manifest_path, region).await?;
        files.push(manifest.data_file);
    }
    anyhow::Ok(files)
}

pub async fn read_bytes(path: &str, region: Option<&str>) -> AnyResult<Cursor<Vec<u8>>> {
    if path.starts_with("s3://") {
        let (access_key, secret_key) = s3::env::get_keys()?;
        let bucket = s3::obj::get_bucket_name_from_path(path)?;
        let store = AmazonS3Builder::from_env()
            .with_region(region.unwrap())
            .with_bucket_name(bucket)
            .with_access_key_id(access_key)
            .with_secret_access_key(secret_key)
            .build()?;
        let obj_key = s3::obj::get_object_key_from_path(path)?;
        let obj_path = object_store::path::Path::from(obj_key.as_str());
        let bytes = store.get(&obj_path).await?.bytes().await?;
        anyhow::Ok(Cursor::new(bytes.to_vec()))
    } else {
        let bytes = std::fs::read(path)?;
        anyhow::Ok(std::io::Cursor::new(bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_datafiles() {
        let metadata = "s3://iceberg-sandbox/mydb/mytable/metadata/00001-b2f7cc94-92e2-4e51-a27d-0164a852d443.metadata.json";
        let datafiles = get_datafiles(metadata, Some("us-east-2"), None).await.unwrap();
        assert_eq!(datafiles.len(), 1);
    }
}
