use dotenvy::dotenv;
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{postgres, Error, Pool, Postgres};
use std::env;
use log::info;
use std::time::Duration;

lazy_static! {
    static ref DB_POOL: OnceCell<Pool<Postgres>> = OnceCell::new();
}

pub async fn init_db() {
    dotenv().ok();

    let db_user = env::var("DB_USER").expect("DB_USER is not set");
    let db_pass = env::var("DB_PASS").expect("DB_PASS is not set");
    let db_host = env::var("DB_HOST").expect("DB_HOST is not set");
    let db_port = env::var("DB_PORT").expect("DB_PORT is not set");
    let db_name = env::var("DB_NAME").expect("DB_NAME is not set");

    let connect_options = PgConnectOptions::new()
        .username(&db_user)
        .password(&db_pass)
        .host(&db_host)
        .port(db_port.parse::<u16>().expect("Invalid DB_PORT"))
        .database(&db_name);

    match PgPoolOptions::new()
        .max_connections(16)
        .min_connections(4)
        .max_lifetime(Duration::from_secs(20_000))
        .idle_timeout(Duration::from_secs(5_000))
        .connect_with(connect_options)
        .await
    {
        Ok(pool) => {
            DB_POOL.set(pool).expect("Failed to set DB_POOL");
            info!("Database initialized success");
        }
        Err(err) => panic!("Failed to create connection pool: {}", err),
    }
}

pub fn get_pool() -> &'static Pool<Postgres> {
    DB_POOL.get().expect("Database pool is not initialized")
}

pub async fn query_task(script: &str) -> Result<postgres::PgQueryResult, Error> {
    let pool = get_pool();
    sqlx::query(script).execute(pool).await
}
