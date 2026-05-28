use anyhow::Result as AnyResult;

use iceberg_inspect::parse;
use comfy_table::*;


pub async fn show_schema(table: &str, region: Option<&str>, snapshot_id: Option<&str>) -> AnyResult<()> {
    let metadata = parse::metadata::TableMetadata::from_file(table, region).await?;
    let snapshot = metadata.get_snapshot(snapshot_id);
        
    if let Some(snapshot_unwrapped) = snapshot {
        if let Some(schema_id) = snapshot_unwrapped.schema_id {
            if let Some(schemas) = metadata.schemas {
                if let Some(schema) = schemas.into_iter().find(|s| s.schema_id == Some(schema_id)) {
                    if let Some(fields) = schema.fields {
                        let mut table = comfy_table::Table::new();
                        table
                            .load_preset(comfy_table::presets::UTF8_FULL)
                            .set_content_arrangement(ContentArrangement::Dynamic)
                            .set_header(vec!["Field ID", "Name", "Type", "Required", "Doc"]);

                        for field in fields {
                            table.add_row(vec![
                                format!("{:?}", field.id.unwrap_or_default()),
                                field.name.unwrap_or_default(),
                                field._type.unwrap_or_default(),
                                format!("{:?}", field.required.unwrap_or_default()),
                                field.doc.unwrap_or_default(),
                            ]);
                        }

                        println!("{}", table);
                    } else {
                        anyhow::bail!("No fields found for schema ID: {}", schema_id)
                    }
                } else {
                    anyhow::bail!("No matching schema found for schema ID: {}", schema_id);
                }
            } else {
                anyhow::bail!("No schemas found in metadata");
            }
        } else {
            anyhow::bail!("No schema ID found for snapshot");
        }
    } else {
        anyhow::bail!("No matching snapshot found");
    }

    anyhow::Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_show_schema() {
        // TODO change to use S3 once that's added later
        let result = show_schema("s3://iceberg-sandbox/mydb/mytable/metadata/00002-a0902076-c7b9-4be2-83e1-5987041a6779.metadata.json", Some("us-east-2"), None);
        assert!(result.await.is_ok());
    }
}