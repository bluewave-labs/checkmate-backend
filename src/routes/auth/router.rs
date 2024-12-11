use crate::core::state::AppGlobalState;
use axum::{
    routing::{get, post},
    Router,
};
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use std::time::Duration;

use super::handlers::{
    check_otp, get_user_detail, register, register_complete, send_email_otp, send_sms_otp, sso,
};

pub fn router() -> Router<AppGlobalState> {
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(5)
        .finish()
        .unwrap();

    Router::new()
        .route("/sso", post(sso))
        .route("/register", post(register))
        .route("/check/otp", post(check_otp))
        .route("/send/sms", get(send_sms_otp))
        .route("/send/email", get(send_email_otp))
        .route("/register/complete/:user_id", get(register_complete))
        .route("/user/detail", get(get_user_detail))
        .layer(GovernorLayer::new(governor_conf))
}
