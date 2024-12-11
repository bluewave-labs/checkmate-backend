use lambda_http::tracing;
use sqlx::PgPool;
use uuid::Uuid;

use crate::core::error::AppError;
use crate::core::helpers::validate_profile_url;
use crate::routes::auth::models::{
    FormCheckResponse, RegisterBodyModel, SSOBodyModel, UserModel, UserResponse,
};

pub async fn get_user_info(pool: &PgPool, user_id: &Uuid) -> Result<UserModel, AppError> {
    let response = sqlx::query_as!(
        UserModel,
        r#"
        SELECT id, first_name, last_name, email, phone, sso_provider, sso_id, is_active 
        FROM "user" where id=$1
        "#,
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_| AppError::NotFound)?;

    Ok(response)
}

pub async fn check_sso(pool: &PgPool, body: &SSOBodyModel) -> Result<UserResponse, AppError> {
    tracing::info!("Body: {:?}", body);
    let response = sqlx::query_as!(
        UserResponse,
        r#"
        SELECT 
            user_id,
            is_new,
            is_active
        FROM sso_sign_in($1, $2, $3, $4, $5, $6)
        "#,
        body.email,
        body.first_name,
        body.last_name,
        body.id,
        body.sso_provider,
        body.photo
    )
    .fetch_one(pool)
    .await
    .map_err(|_| AppError::NotFound)?;

    Ok(response)
}

pub async fn check_register_field(
    pool: &PgPool,
    email: &str,
    phone: &str,
    user_id: &Uuid,
) -> Result<FormCheckResponse, AppError> {
    let response = sqlx::query_as!(
        FormCheckResponse,
        r#"
        SELECT
        CASE
            WHEN EXISTS (SELECT 1 FROM "user" WHERE email = $1 and id!=$3) THEN 'email'
            WHEN EXISTS (SELECT 1 FROM "user" WHERE phone = $2 and id!=$3) THEN 'phone'
            ELSE NULL
        END as existing_field
        "#,
        email,
        phone,
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_| AppError::DatabaseError)?;

    Ok(response)
}

pub async fn update_user(pool: &PgPool, body: &RegisterBodyModel) -> Result<(), AppError> {
    tracing::info!("Body: {:?}", body);
    sqlx::query!(
        r#"
        UPDATE "user" 
        SET first_name=$1, last_name=$2, email=$3, phone=$4, last_login=now() 
        WHERE id=$5
        "#,
        body.first_name,
        body.last_name,
        body.email,
        body.phone,
        body.id
    )
    .execute(pool)
    .await
    .map_err(|_| AppError::DatabaseError)?;

    Ok(())
}

pub async fn update_user_register_ok(pool: &PgPool, user_id: &Uuid) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        UPDATE "user" 
        SET is_active=TRUE, last_login=now() 
        WHERE id=$1
        "#,
        user_id
    )
    .execute(pool)
    .await
    .map_err(|_| AppError::DatabaseError)?;

    Ok(())
}

pub async fn insert_user(pool: &PgPool, body: &RegisterBodyModel) -> Result<(), AppError> {
    tracing::info!("Body: {:?}", body);
    sqlx::query!(
        r#"
        INSERT INTO "user" (
            id, 
            first_name, 
            last_name, 
            email, 
            phone, 
            password,
            is_active,
            date_joined,
            last_login
        ) 
        VALUES ($1, $2, $3, $4, $5, $6, true, now(), now())
        "#,
        body.id,
        body.first_name,
        body.last_name,
        body.email,
        body.phone,
        body.password,
    )
    .execute(pool)
    .await
    .map_err(|_| AppError::DatabaseError)?;

    Ok(())
}
