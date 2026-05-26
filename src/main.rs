mod parse;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to a manifest.json file
    #[arg(short, long)]
    table: String,
    
    /// Command to execute [snapshots, schema, files, read]
    #[arg(short, long)]
    command: String,
}

fn main() {
    let args = Args::parse();
    
    let metadata = parse::TableMetadata::from_file(&args.table)
        .expect("Failed to parse table metadata");
    println!("Table Metadata: {:?}\n\n", metadata);
    
    let cur_snapshot = metadata.get_current_snapshot()
        .expect("Failed to get current snapshot");
    println!("Current Snapshot: {:?}\n\n", cur_snapshot);
    
    let manifest_list_path = cur_snapshot.get_manifest_list_path()
        .expect("Failed to get manifest list path from current snapshot");
    println!("Manifest List Path: {:?}\n\n", manifest_list_path);
}
