# iceberg-inspect

A CLI tool for inspecting [Apache Iceberg](https://iceberg.apache.org/) tables from the command line. Reads table metadata directly from the filesystem and displays snapshots, schema, and (soon) data files and row-level content in a formatted terminal table.

## Installation

```bash
cargo build --release
# binary is at target/release/iceberg-inspect
```

## Usage

```
iceberg-inspect --table <path-to-metadata.json> --command <command> [--snapshot <snapshot-id>]
```

### Commands

| Command     | Description                                         |
|-------------|-----------------------------------------------------|
| `snapshots` | List all snapshots with ID, sequence number, parent, and timestamp |
| `schema`    | Show the schema for a snapshot (defaults to the latest) |
| `files`     | *(not yet implemented)* List data files in a snapshot |
| `read`      | *(not yet implemented)* Read rows from a snapshot   |

### Options

| Flag              | Description                                                       |
|-------------------|-------------------------------------------------------------------|
| `-t, --table`     | Path to the table's `metadata.json` file (required)              |
| `-c, --command`   | Command to run (required)                                         |
| `-s, --snapshot`  | Snapshot ID to inspect (optional; defaults to the latest)        |

### Examples

List all snapshots for a table:

```bash
iceberg-inspect --table /path/to/warehouse/my_table/metadata/v3.metadata.json --command snapshots
```

Show the schema for the latest snapshot:

```bash
iceberg-inspect --table /path/to/warehouse/my_table/metadata/v3.metadata.json --command schema
```

Show the schema for a specific snapshot:

```bash
iceberg-inspect --table /path/to/warehouse/my_table/metadata/v3.metadata.json \
  --command schema \
  --snapshot 8728349182736491
```

## Roadmap

- [ ] `files` command — list data files for a snapshot
- [ ] `read` command — display rows from Parquet data files
- [ ] S3/object-store support for remote tables
- [ ] Filter snapshots by timestamp or sequence number

## Dependencies

- [arrow](https://crates.io/crates/arrow) / [parquet](https://crates.io/crates/parquet) — reading Parquet data files
- [apache-avro](https://crates.io/crates/apache-avro) — reading Avro manifest lists and manifest files
- [object_store](https://crates.io/crates/object_store) — local and S3 storage backends
- [clap](https://crates.io/crates/clap) — CLI argument parsing
- [comfy-table](https://crates.io/crates/comfy-table) — terminal table formatting
- [anyhow](https://crates.io/crates/anyhow) — error handling
