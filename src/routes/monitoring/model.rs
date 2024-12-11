use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct MonitoringRequest {
    pub url: String,
    pub name: String,
    pub check_interval: i32,
    pub is_active: bool,
    pub ssl_check: SSLCheckRequest,
    pub advanced_settings: AdvancedSettingsRequest,
    pub notification_methods: Vec<i32>,
    pub request_headers: Vec<RequestHeaderRequest>,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SSLCheckRequest {
    pub check_ssl_errors: bool,
    pub ssl_expiry_reminder: bool,
    pub domain_expiry_reminder: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdvancedSettingsRequest {
    pub request_timeout: i32,
    pub follow_redirects: bool,
    pub auth_type_id: Option<i32>,
    pub auth_username: Option<String>,
    pub auth_password: Option<String>,
    pub http_method: String,
    pub request_body: Option<String>,
    pub send_as_json: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestHeaderRequest {
    pub name: String,
    pub value: String,
}
