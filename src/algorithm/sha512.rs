use sha2::{Sha512, Digest};

pub async fn sha512(data: &str) -> Result<String, String> {
    if data.trim().is_empty() {
        return Err("Data cannot be empty".to_string());
    }

    let mut hasher = Sha512::new();
    
    hasher.update(data.as_bytes());

    let result = hasher.finalize();

    Ok(format!("{:x}", result))
}