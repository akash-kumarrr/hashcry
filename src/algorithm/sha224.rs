use sha2::Sha224;
use sha2::Digest;

pub async fn sha224(data: &str) -> Result<String, String> {
    if data.trim().is_empty() {
        return Err("Data cannot be empty".to_string());
    }

    let mut hasher = Sha224::new();
    
    hasher.update(data.as_bytes());

    let result = hasher.finalize();

    Ok(format!("{:x}", result))
}