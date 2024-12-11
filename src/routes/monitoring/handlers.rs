use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use serde_json::{json, Value};
use crate::core::state::AppGlobalState;
use crate::monitoring::models::{MonitoringRequest, Monitor, AuthTypes, NotificationMethod, AcceptedStatusCode, SSLCheck, AdvancedSettings, MonitorNotification, RequestHeader, MonitorTag};
use crate::core::error::AppError;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_monitoring_record(
    AuthClaims(token_data): AuthClaims,
    State(pool): State<AppGlobalState>,
    Json(monitoring_data): Json<MonitoringRequest>,
) -> Result<impl IntoResponse, AppError> {

    if let Some(user_id) = token_data.data.id {
        let monitor_id = Uuid::new_v4();

        sqlx::query!(
            r#"
            INSERT INTO monitors (id, user_id, url, name, check_interval, is_active)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            monitor_id,
            user_id,
            monitoring_data.url,
            monitoring_data.name,
            monitoring_data.check_interval,
            monitoring_data.is_active
        )
        .execute(&pool.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        sqlx::query!(
            r#"
            INSERT INTO ssl_checks (monitor_id, check_ssl_errors, ssl_expiry_reminder, domain_expiry_reminder)
            VALUES ($1, $2, $3, $4)
            "#,
            monitor_id,
            monitoring_data.ssl_check.check_ssl_errors,
            monitoring_data.ssl_check.ssl_expiry_reminder,
            monitoring_data.ssl_check.domain_expiry_reminder
        )
        .execute(&pool.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        sqlx::query!(
            r#"
            INSERT INTO advanced_settings (monitor_id, request_timeout, follow_redirects, auth_type_id, auth_username, auth_password, http_method, request_body, send_as_json)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            monitor_id,
            monitoring_data.advanced_settings.request_timeout,
            monitoring_data.advanced_settings.follow_redirects,
            monitoring_data.advanced_settings.auth_type_id,
            monitoring_data.advanced_settings.auth_username,
            monitoring_data.advanced_settings.auth_password,
            monitoring_data.advanced_settings.http_method,
            monitoring_data.advanced_settings.request_body,
            monitoring_data.advanced_settings.send_as_json
        )
        .execute(&pool.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        for method_id in &monitoring_data.notification_methods {
            sqlx::query!(
                r#"
                INSERT INTO monitor_notifications (monitor_id, method_id)
                VALUES ($1, $2)
                "#,
                monitor_id,
                method_id
            )
            .execute(&pool.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        for header in &monitoring_data.request_headers {
            sqlx::query!(
                r#"
                INSERT INTO request_headers (advanced_settings_id, name, value)
                VALUES ($1, $2, $3)
                "#,
                monitor_id, 
                header.name,
                header.value
            )
            .execute(&pool.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        for tag in &monitoring_data.tags {
            sqlx::query!(
                r#"
                INSERT INTO monitor_tags (monitor_id, name)
                VALUES ($1, $2)
                "#,
                monitor_id,
                tag
            )
            .execute(&pool.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        Ok(Json(json!({"message": "Monitoring record created successfully", "id": monitor_id})))
    }
    Err(AppError::Unauthorized("Token Data Not Valid!".to_owned()))
}
