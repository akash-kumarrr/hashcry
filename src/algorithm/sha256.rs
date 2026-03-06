use sha2::{Sha256, Digest};

pub async fn sha256(data: &str) -> Result<String, String> {
    if data.trim().is_empty() {
        return Err("Data cannot be empty".to_string());
    }

    let mut hasher = Sha256::new();
    
    hasher.update(data.as_bytes());

    let result = hasher.finalize();

    Ok(format!("{:x}", result))
}