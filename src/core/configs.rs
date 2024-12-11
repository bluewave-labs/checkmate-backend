use once_cell::sync::Lazy;
use std::env;

pub static DATABASE_URL: Lazy<String> =
    Lazy::new(|| env::var("DATABASE_URL").expect("DATABASE_URL must be set"));
