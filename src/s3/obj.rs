//use object_store::ObjectStore;
use url;
use anyhow::Result as AnyResult;

pub fn get_bucket_name_from_path(path: &str) -> AnyResult<String> {
    let url = url::Url::parse(path).ok().unwrap();
    if url.scheme() == "s3" {
        anyhow::Ok(url.host_str().map(|s| s.to_string()).unwrap())
    } else {
        anyhow::bail!("Failed to parse s3 bucket name.")
    }
}

pub fn get_object_key_from_path(path: &str) -> AnyResult<String> {
    let url = url::Url::parse(path).ok().unwrap();
    if url.scheme() == "s3" {
        anyhow::Ok(url.path().trim_start_matches('/').to_string())
    } else {
        anyhow::bail!("Failed to parse s3 object key.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bucket_name_from_path() {
        let bn = get_bucket_name_from_path("s3://iceberg-sandbox/mydb/mytable/metadata/00002-a0902076-c7b9-4be2-83e1-5987041a6779.metadata.json")
            .expect("Failed to parse bucket name");
        assert_eq!(bn, String::from("iceberg-sandbox"));
    }

    #[test]
    fn test_get_object_key_from_path() {
        let bn = get_object_key_from_path("s3://iceberg-sandbox/mydb/mytable/metadata/00002-a0902076-c7b9-4be2-83e1-5987041a6779.metadata.json")
            .expect("Failed to parse object key");
        assert_eq!(bn, String::from("mydb/mytable/metadata/00002-a0902076-c7b9-4be2-83e1-5987041a6779.metadata.json"));
    }
}