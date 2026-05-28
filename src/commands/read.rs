use iceberg_inspect::parse;
use anyhow::Result as AnyResult;

pub fn read(
    table_path: &str,
    snapshot_id: Option<&str>,
    limit_total: Option<usize>,
    columns: &[String]) -> AnyResult<()> {
    let mut limit = limit_total;
    let data_files = parse::get_datafiles(table_path, snapshot_id)?;

    for data_file in data_files {
        let batches = data_file.to_arrow()?;
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

    #[test]
    fn test_read() {
        let table_path = "./taxis/metadata/00003-3b45d19f-94fb-4ea3-8d77-d769539ba79c.metadata.json";
        read(table_path, None, Some(5), &vec![]).unwrap();
    }
}