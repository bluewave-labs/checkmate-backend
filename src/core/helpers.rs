use super::error::AppError;
use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};

pub fn generate_otp_and_hash(secret: &str) -> (String, String) {
    let otp: u32 = 1234; //rand::thread_rng().gen_range(100_000..1_000_000);
    let otp_str = otp.to_string();

    let now: DateTime<Utc> = Utc::now();
    let current_hour = now.format("%Y%m%d%h").to_string();

    let combined = format!("{}{}{}", otp_str, current_hour, secret);

    let mut hasher = Sha256::new();
    hasher.update(combined);
    let hash = format!("{:x}", hasher.finalize());

    (otp_str, hash)
}

pub fn verify_otp(otp: &str, hash: &str, secret: &str) -> bool {
    let now: DateTime<Utc> = Utc::now();
    let current_hour = now.format("%Y%m%d%h").to_string();

    let combined = format!("{}{}{}", otp, current_hour, secret);

    let mut hasher = Sha256::new();
    hasher.update(combined);
    let calculated_hash = format!("{:x}", hasher.finalize());

    calculated_hash == hash
}

pub fn validate_profile_url(url: &str) -> Result<(), AppError> {
    if !url.starts_with("https://") && url != "" {
        Err(AppError::FormatError(
            "Profile URL must start with 'https://'".to_string(),
        ))
    } else {
        Ok(())
    }
}
