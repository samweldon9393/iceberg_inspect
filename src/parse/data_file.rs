use arrow::util::pretty::print_batches;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use std::fs::File;
use anyhow::Result as AnyResult;

#[derive(Debug, Clone, serde::Deserialize)]
#[allow(dead_code)]
pub struct DataFileRecord {
    pub content: i32,
    pub file_path: String,
    pub file_format: String,
    pub partition_data: Option<String>,
    pub record_count: i64,
    pub file_size_in_bytes: i64,
    //pub column_sizes: Option<Vec<ColumnSize>>,
}

impl DataFileRecord {
    pub fn to_arrow(&self) -> AnyResult<Vec<arrow::record_batch::RecordBatch>> {
        let file = File::open(&self.file_path)?;
        let reader = ParquetRecordBatchReaderBuilder::try_new(file)?.build()?;
        let batches = reader
            .collect::<AnyResult<Vec<arrow::record_batch::RecordBatch>, arrow::error::ArrowError>>()?;
        Ok(batches)
    }
    pub fn print_parquet_file(&self) -> AnyResult<()> {
        let batches = self.to_arrow()?;
        print_batches(&batches)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_arrow() {
        let data_file = DataFileRecord {
            content: 0, //unused
            file_path: "./taxis/data/00000-1-d6877f7e-bceb-480d-a2a0-d63fbe20045f.parquet".to_string(),
            file_format: "PARQUET".to_string(),
            partition_data: None, //unused
            record_count: 0, // unused
            file_size_in_bytes: 3570920,
        };
        let batches = data_file.to_arrow().unwrap();
        assert!(!batches.is_empty());
    }
}