use axum::{
    routing::post,
    Router,
};
use crate::monitoring::handlers::create_monitoring_record;
use crate::core::state::AppGlobalState;

pub fn router() -> Router<AppGlobalState> {
    Router::new()
        .route("/monitoring", post(create_monitoring_record))
}
