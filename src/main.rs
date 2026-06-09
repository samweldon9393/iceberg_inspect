/**
 * main.rs
 * 
 * This is the main entry point for the Iceberg inspect tool.
 */

mod commands;

use clap::Parser;
use anyhow::Result as AnyResult;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to a manifest.json file
    #[arg(short, long)]
    table: String,
    
    /// Command to execute [snapshots, schema, files, read]
    #[arg(short, long)]
    command: String,
    
    /// Snapshot to inspect (optional, used for read, schema and files commands)
    #[arg(short, long)]
    snapshot: Option<String>,
    
    /// Number of rows to print (optional, used for read command)
    #[arg(long)]
    limit: Option<usize>,

    /// Columns to print, enter comma separated list (optional, used for read command)
    #[arg(long, value_delimiter = ',')]
    columns: Vec<String>,
    
    /// S3 bucket region (optional, used when files are in S3)
    #[arg(short, long)]
    region: Option<String>,
}

#[tokio::main]
async fn main() -> AnyResult<()> {
    let args = Args::parse();

    match args.command.as_str() {
        "snapshots" => commands::list_snapshots(&args.table, args.region.as_deref()).await,
        "schema" => commands::show_schema(&args.table, args.region.as_deref(), args.snapshot.as_deref()).await,
        "files" => commands::list_files(&args.table, args.region.as_deref(), args.snapshot.as_deref()).await,
        "read" => commands::read(&args.table, args.region.as_deref(), args.snapshot.as_deref(), args.limit, &args.columns).await,
        _ => anyhow::bail!("Unknown command: {}", args.command),
    } 
}
