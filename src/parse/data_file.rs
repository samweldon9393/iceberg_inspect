use arrow::util::pretty::print_batches;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use parquet::errors::Result;
use std::fs::File;

pub fn read_parquet_file(path: &str) -> Result<()> {
    let file = File::open(path)?;
    let reader = ParquetRecordBatchReaderBuilder::try_new(file)?.build()?;
    let batches = reader
        .collect::<Result<Vec<arrow::record_batch::RecordBatch>, arrow::error::ArrowError>>()?;
    print_batches(&batches)?;
    Ok(())
}