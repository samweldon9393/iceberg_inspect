use iceberg_inspect::parse;

#[tokio::test]
async fn test_full_parse() {
    let metadata = parse::TableMetadata::from_file("./taxis/metadata/00003-3b45d19f-94fb-4ea3-8d77-d769539ba79c.metadata.json", Some(""))
        .await.expect("Failed to parse table metadata");
    println!("Table Metadata: {:?}\n\n", metadata);
    
    let cur_snapshot = metadata.get_current_snapshot()
        .expect("Failed to get current snapshot");
    println!("Current Snapshot: {:?}\n\n", cur_snapshot);
    
    let manifest_list_path = cur_snapshot.get_manifest_list_path()
        .expect("Failed to get manifest list path from current snapshot");
    println!("Manifest List Path: {:?}\n\n", manifest_list_path);
    
    let manifest_list = parse::ManifestList::from_file(&manifest_list_path, Some(""))
        .await.expect("Failed to parse manifest list");
    println!("Manifest List Records: {:?}\n\n", manifest_list.records);
    
    let first_record = manifest_list.records.first()
        .expect("No records found in manifest list");
    println!("First Manifest List Record: {:?}\n\n", first_record);
    
let manifest = parse::ManifestFile::from_file(&first_record.manifest_path,Some(""))
        .await.unwrap();
    println!("Manifest File: {:?}\n\n", manifest);

    let parquet_path = manifest.data_file
        .file_path.clone();
    println!("First Parquet File Path: {:?}\n\n", parquet_path);
    
    let data_file = manifest.data_file;
    println!("Data File Record: {:?}\n\n", data_file);
}