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

/*
pub fn get_s3_store(path: &str) -> AnyResult<dyn ObjectStore> {
    let bucket = get_bucket_name_from_path(path)?;
    AmazonS3Builder::from_env().with_bucket_name(bucket).build()
}
*/