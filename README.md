# iceberg-inspect

A CLI tool for inspecting [Apache Iceberg](https://iceberg.apache.org/) tables from the command line. Reads table metadata directly from the filesystem and displays snapshots, schema, data files and row-level content in a formatted terminal table.

## Installation

```bash
cargo build --release
# binary is at target/release/iceberg-inspect
```

## Usage

```
iceberg-inspect --table <path-to-metadata.json> --command <command> [--snapshot <snapshot-id> --limit <num-rows> --columns <column,names> --region <S3-region>]
```

### Commands

| Command     | Description                                         |
|-------------|-----------------------------------------------------|
| `snapshots` | List all snapshots with ID, sequence number, parent, and timestamp |
| `schema`    | Show the schema for a snapshot (defaults to the latest) |
| `files`     | List data files in a snapshot |
| `read`      | Read rows from a snapshot   |

### Options

| Flag              | Description                                                       |
|-------------------|-------------------------------------------------------------------|
| `-t, --table`     | Path to the table's `metadata.json` file (required)              |
| `-c, --command`   | Command to run (required)                                         |
| `-s, --snapshot`  | Snapshot ID to inspect (optional; defaults to the latest)        |
| `--columns`   | Collumns to print (optional, used for read command; defaults to all columns)                                         |
| `--limit`  | Max number of rows to print (optional, used for read command; defaults to all rows)        |
| `-r, --region`  | Amazon S3 region (only used with S3 buckets)        |

### Examples

List all snapshots for a table in S3:

```bash
iceberg-inspect --table s3://path/to/warehouse/my_table/metadata/v3.metadata.json -r us-east-1 --command snapshots
```

Show the schema for the latest snapshot for a local table:

```bash
iceberg-inspect --table /path/to/warehouse/my_table/metadata/v3.metadata.json --command schema
```

List the data files in a specific snapshot:

```bash
iceberg-inspect --table /path/to/warehouse/my_table/metadata/v3.metadata.json \
  --command files \
  --snapshot 8728349182736491
```

Print 10 rows, project onto three columns:

```bash
iceberg-inspect --table s3://path/to/warehouse/my_table/metadata/v3.metadata.json \
  --region us-east-1 \
  --command read \
  --columns VendorID,fare_amount,extra
  --limit 10
```
 

## Roadmap

- [ ] Filter snapshots by timestamp or sequence number

## Dependencies

- [arrow](https://crates.io/crates/arrow) / [parquet](https://crates.io/crates/parquet) — reading Parquet data files
- [apache-avro](https://crates.io/crates/apache-avro) — reading Avro manifest lists and manifest files
- [object_store](https://crates.io/crates/object_store) — local and S3 storage backends
- [clap](https://crates.io/crates/clap) — CLI argument parsing
- [comfy-table](https://crates.io/crates/comfy-table) — terminal table formatting
- [anyhow](https://crates.io/crates/anyhow) — error handling
