use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::libcore::configs::DATABASE_URL;

pub async fn init_db() -> PgPool {
    // let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&DATABASE_URL)
        .await
    {
        Ok(pool) => pool,
        Err(err) => {
            println!("❌ Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };
    return pool;
}
