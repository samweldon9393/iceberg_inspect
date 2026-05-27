use anyhow::Result as AnyResult;

use iceberg_inspect::parse;
use comfy_table::*;


pub fn show_schema(table: &str, snapshot_id: Option<&str>) -> AnyResult<()> {
    let metadata = parse::metadata::TableMetadata::from_file(table)?;
    println!("Table UUID: {:?}", metadata.table_uuid.unwrap_or_default());
    
    let snapshots = metadata.snapshots.unwrap_or_default();
    let snapshot = if let Some(sid) = snapshot_id {
        snapshots.into_iter().find(|s| s.snapshot_id.as_ref().map(|id| id.to_string()) == Some(sid.to_string()))
    } else {
        snapshots.into_iter().max_by_key(|s| s.timestamp_ms.unwrap_or_default())
    };

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