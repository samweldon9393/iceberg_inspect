use arrow::util::pretty::print_batches;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use anyhow::Result as AnyResult;

use crate::parse;

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
    pub async fn to_arrow(&self, region: Option<&str>) -> AnyResult<Vec<arrow::record_batch::RecordBatch>> {
        let cursor = parse::read_bytes(&self.file_path, region).await?;
        let bytes = bytes::Bytes::from(cursor.into_inner());
        let reader = ParquetRecordBatchReaderBuilder::try_new(bytes)?.build()?;
        let batches = reader
            .collect::<AnyResult<Vec<arrow::record_batch::RecordBatch>, arrow::error::ArrowError>>()?;
        Ok(batches)
    }
    #[allow(dead_code)]
    pub async fn print_parquet_file(&self, region: Option<&str>) -> AnyResult<()> {
        let batches = self.to_arrow(region).await?;
        print_batches(&batches)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_to_arrow() {
        let data_file = DataFileRecord {
            content: 0, //unused
            file_path: "s3://iceberg-sandbox/bauplan_tpcc_1_spine_setup.parquet".to_string(),
            file_format: "PARQUET".to_string(),
            partition_data: None, //unused
            record_count: 0, // unused
            file_size_in_bytes: 3570920,
        };
        let batches = data_file.to_arrow(Some("us-east-2")).await.unwrap();
        assert!(!batches.is_empty());
    }
}