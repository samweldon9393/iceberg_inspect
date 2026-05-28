use iceberg_inspect::parse;
use anyhow::Result as AnyResult;

pub async fn read(
    table_path: &str,
    region: Option<&str>,
    snapshot_id: Option<&str>,
    limit_total: Option<usize>,
    columns: &[String]) -> AnyResult<()> {
    let mut limit = limit_total;
    let data_files = parse::get_datafiles(table_path, region, snapshot_id).await?;

    for data_file in data_files {
        let batches = data_file.to_arrow(region).await?;
        for batch in batches {
            let selected_batch = if columns.is_empty() {
                batch
            } else {
                let column_indices: Vec<usize> = columns.iter()
                    .filter_map(|col| batch.schema().index_of(col).ok())
                    .collect();
                batch.project(&column_indices)?
            };
            let limited_batch = if let Some(lim) = limit {
                selected_batch.slice(0, lim)
            } else {
                selected_batch
            };
            let printed_count = limited_batch.num_rows();
            arrow::util::pretty::print_batches(&[limited_batch])?;
            if let Some(lim) = limit {
                if lim > printed_count {
                    limit = Some(lim - printed_count);
                } else {
                    return Ok(());
                }
            }
        }
    }

    anyhow::Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_read() {
        let table_path = "s3://iceberg-sandbox/mydb/mytable/metadata/00002-a0902076-c7b9-4be2-83e1-5987041a6779.metadata.json";
        read(table_path, Some("us-east-2"), None, Some(5), &vec![]).await.unwrap();
    }
}