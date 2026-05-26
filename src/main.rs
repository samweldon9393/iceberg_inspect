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
    println!("Table: {}", args.table);
    
    let parsed = parse::TableMetadata::from_file(&args.table);
    match parsed {
        Ok(metadata) => {
            println!("Parsed metadata: {:?}", metadata);
            // Here you can add logic to handle different commands based on args.command
        },
        Err(e) => {
            eprintln!("Failed to parse metadata: {}", e);
        }
    }
}
