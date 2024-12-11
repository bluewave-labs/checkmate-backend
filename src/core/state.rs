use sqlx::PgPool;

#[derive(Clone)]
pub struct AppGlobalState {
    pub pool: PgPool,
}
