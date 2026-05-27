mod parse;
mod commands;

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

    match args.command.as_str() {
        "snapshots" => commands::list_snapshots(&args.table),
        "schema" => println!("Schema command not implemented yet"),
        "files" => println!("Files command not implemented yet"),
        "read" => println!("Read command not implemented yet"),
        _ => println!("Unknown command: {}", args.command),
    } 
}
