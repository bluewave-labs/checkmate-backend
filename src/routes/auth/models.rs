use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct OTPBody {
    pub otp: String,
    pub user_id: Uuid,
    pub hash_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserIdBody {
    pub user_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SSOBodyModel {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub photo: String,
    pub email: String,
    pub sso_provider: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterBodyModel {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct UserModel {
    pub id: Option<Uuid>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password: Option<String>,
    pub sso_provider: Option<String>,
    pub sso_id: Option<String>,
    pub is_active: Option<bool>,
    pub is_staff: Option<bool>,
    pub is_superuser: Option<bool>,
    pub date_joined: Option<OffsetDateTime>,
    pub last_login: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct UserResponse {
    pub user_id: Option<Uuid>,
    pub is_new: Option<bool>,
    pub is_active: Option<bool>,
}

impl IntoResponse for UserResponse {
    fn into_response(self) -> axum::response::Response {
        let status = if self.is_new.unwrap_or(false) {
            StatusCode::CREATED
        } else {
            StatusCode::OK
        };
        (status, Json(self)).into_response()
    }
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct FormCheckResponse {
    pub existing_field: Option<String>,
}

impl IntoResponse for FormCheckResponse {
    fn into_response(self) -> axum::response::Response {
        let status = if self.existing_field.as_deref().unwrap_or("") == "" {
            StatusCode::OK
        } else {
            StatusCode::CONFLICT
        };
        (status, Json(self)).into_response()
    }
}
