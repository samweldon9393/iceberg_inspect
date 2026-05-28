/**
 * main.rs
 * 
 * This is the main entry point for the Iceberg inspect tool.
 */

mod parse;
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
}

fn main() -> AnyResult<()> {
    let args = Args::parse();

    match args.command.as_str() {
        "snapshots" => commands::list_snapshots(&args.table),
        "schema" => commands::show_schema(&args.table, args.snapshot.as_deref()),
        "files" => commands::list_files(&args.table, args.snapshot.as_deref()),
        "read" => commands::read(&args.table, args.snapshot.as_deref(), args.limit, &args.columns),
        _ => anyhow::bail!("Unknown command: {}", args.command),
    } 
}
