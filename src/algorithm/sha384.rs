use sha2::{Sha384, Digest};

pub async fn sha384(data: &str) -> Result<String, String> {
    if data.trim().is_empty() {
        return Err("Data cannot be empty".to_string());
    }

    let mut hasher = Sha384::new();
    
    hasher.update(data.as_bytes());

    let result = hasher.finalize();

    Ok(format!("{:x}", result))
}