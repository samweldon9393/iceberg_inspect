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
    
    // Snapshot to inspect (optional, used for read, schema and files commands)
    #[arg(short, long)]
    snapshot: Option<String>,
}

fn main() -> AnyResult<()> {
    let args = Args::parse();

    match args.command.as_str() {
        "snapshots" => commands::list_snapshots(&args.table),
        "schema" => commands::show_schema(&args.table, args.snapshot.as_deref()),
        "files" => commands::list_files(&args.table, args.snapshot.as_deref()),
        "read" => anyhow::bail!("Read command not implemented yet"),
        _ => anyhow::bail!("Unknown command: {}", args.command),
    } 
}
