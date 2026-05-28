use anyhow::Result as AnyResult;

pub fn get_keys() -> AnyResult<(String, String)> {
    let access_key = std::env::var("AWS_ACCESS_KEY_ID")
                                .map_err(|_| anyhow::anyhow!("AWS_ACCESS_KEY_ID not set"))?;
    let secret_key = std::env::var("AWS_SECRET_ACCESS_KEY")
                                .map_err(|_| anyhow::anyhow!("AWS_SECRET_ACCESS_KEY not set"))?;
    Ok((access_key, secret_key))
}