use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::core::configs::JWT_KEYS;

#[derive(Serialize, Deserialize, Debug)]
pub struct JwtClaim {
    pub data: crate::routes::auth::models::UserModel,
    pub iat: usize,
    pub exp: usize,
}

impl JwtClaim {
    pub fn new(data: crate::routes::auth::models::UserModel) -> Self {
        let expiration = Utc::now()
            .checked_add_signed(Duration::days(200))
            .expect("valid timestamp")
            .timestamp();

        JwtClaim {
            data,
            exp: expiration as usize,
            iat: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize,
        }
    }
    pub fn encode_token(&self) -> String {
        encode(
            &Header::default(),
            &self,
            &JWT_KEYS.0, // &EncodingKey::from_secret(secret.as_ref()),
        )
        .unwrap()
    }
}

pub fn verify_token(token: &str) -> Result<JwtClaim, jsonwebtoken::errors::Error> {
    let validation = Validation::default();
    let token_data = decode::<JwtClaim>(token, &JWT_KEYS.1, &validation)?;
    Ok(token_data.claims)
}
