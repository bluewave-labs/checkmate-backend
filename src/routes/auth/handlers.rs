use crate::core::dal::auth::auth::{
    check_register_field, get_user_info, update_user_register_ok,insert_user,
};
use crate::core::error::AppError;
use crate::core::helpers::{generate_otp_and_hash, verify_otp};
use crate::core::libs::AuthClaims;
use crate::core::models::jwt::JwtClaim;
use crate::core::state::AppGlobalState;
use crate::{
    auth::models::SSOBodyModel,
    core::dal::auth::auth::{check_sso},
};

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use super::models::{OTPBody, RegisterBodyModel};

async fn get_user_model_response(
    pool: &PgPool,
    user_id: &Uuid,
    sms_hash: &str,
    email_hash: &str,
) -> Result<Json<Value>, AppError> {
    let user_model = get_user_info(&pool, user_id).await?;
    let jwt = JwtClaim::new(user_model.clone()).encode_token();
    Ok(Json(
        json!({"data": user_model, "token": jwt, "sms_hash":sms_hash, "email_hash":email_hash}),
    ))
}

pub async fn sso(
    State(pool): State<AppGlobalState>,
    Json(sso_model): Json<SSOBodyModel>,
) -> Result<Json<Value>, AppError> {
    tracing::info!("{:?}", sso_model);
    let response = check_sso(&pool.pool, &sso_model).await?;
    return get_user_model_response(
        &pool.pool,
        &response.user_id.unwrap_or(Uuid::new_v4()),
        "",
        "",
    )
    .await;
}

pub async fn register(
    State(pool): State<AppGlobalState>,
    Json(register_model): Json<RegisterBodyModel>,
) -> Result<Json<Value>, AppError> {
    tracing::info!("{:?}", register_model);
    let response = check_register_field(
        &pool.pool,
        &register_model.email,
        &register_model.phone.as_deref().unwrap_or(""),
        &register_model.id,
    )
    .await?;
    if let Some(existing_field) = response.existing_field {
        let error_message = match existing_field.as_str() {
            "email" => AppError::Conflict("Email".to_owned()),
            "phone" => AppError::Conflict("Phone".to_owned()),
            _ => AppError::Conflict("Unknown".to_owned()),
        };
        return Err(error_message);
    }
    // update user
    insert_user(&pool.pool, &register_model).await?;

    //OTP
    // SMS
    let (sms_otp, sms_hash) = generate_otp_and_hash(&register_model.id.to_string());
    tracing::info!("SMS: {}", sms_otp);
    // EMAIL
    let (email_otp, email_hash) = generate_otp_and_hash(&register_model.id.to_string());
    tracing::info!("Email: {}", email_otp);

    return get_user_model_response(&pool.pool, &register_model.id, &sms_hash, &email_hash).await;
}

pub async fn check_otp(Json(otp_model): Json<OTPBody>) -> impl IntoResponse {
    tracing::info!("{:?}", otp_model);

    let ok = verify_otp(
        &otp_model.otp,
        &otp_model.hash_code,
        &otp_model.user_id.to_string(),
    );
    match ok {
        true => (StatusCode::OK, Json("OK")).into_response(),
        false => AppError::Unauthorized("Invalid OTP Code".to_owned()).into_response(),
    }
}

pub async fn send_sms_otp(AuthClaims(token_data): AuthClaims) -> impl IntoResponse {
    if let Some(user_id) = token_data.data.id {
        println!("{}", user_id);
        let (sms_otp, sms_hash) = generate_otp_and_hash(&user_id.to_string());
        tracing::info!("SMS:{}", sms_otp);
        return Json(json!({"sms_hash": sms_hash})).into_response();
    }
    AppError::Unauthorized("Token Data Not Valid".to_owned()).into_response()
}

pub async fn send_email_otp(AuthClaims(token_data): AuthClaims) -> impl IntoResponse {
    if let Some(user_id) = token_data.data.id {
        println!("{}", user_id);
        let (email_otp, email_hash) = generate_otp_and_hash(&user_id.to_string());
        tracing::info!("EMAIL:{}", email_otp);
        return Json(json!({"email_hash": email_hash})).into_response();
    }
    AppError::Unauthorized("Token Data Not Valid".to_owned()).into_response()
}

pub async fn register_complete(
    Path(user_id): Path<Uuid>,
    State(pool): State<AppGlobalState>,
) -> Result<Json<Value>, AppError> {
    update_creator_register_ok(&pool.pool, &user_id).await?;
    return Ok(Json(json!({"message":""})));
}

pub async fn get_user_detail(
    AuthClaims(token_data): AuthClaims,
    State(pool): State<AppGlobalState>,
) -> Result<Json<Value>, AppError> {
    if let Some(user_id) = token_data.data.id {
        println!("{}", user_id);
        return get_user_model_response(&pool.pool, &user_id, "", "").await;
    }
    Err(AppError::Unauthorized("Token Data Not Valid".to_owned()))
}
