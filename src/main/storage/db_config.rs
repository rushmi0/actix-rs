use std::cell::OnceCell;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::env;

/// ใช้ `OnceCell` เพื่อเก็บข้อมูล Pool แบบ Lazy Static
static DB_POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

/// กำหนดค่าการเชื่อมต่อฐานข้อมูลและสร้าง Pool
pub fn initialize_db() {
    dotenv().ok(); // โหลดค่าจากไฟล์ .env

    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        env::var("DB_USER").expect("DB_USER is not set"),
        env::var("DB_PASS").expect("DB_PASS is not set"),
        env::var("DB_HOST").expect("DB_HOST is not set"),
        env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string()),
        env::var("DB_NAME").expect("DB_NAME is not set"),
    );

    let pool = PgPoolOptions::new()
        .max_connections(32) // จำนวนการเชื่อมต่อสูงสุด
        .min_connections(16) // การเชื่อมต่อขั้นต่ำ
        .max_lifetime(std::time::Duration::from_secs(2_000_000))
        .idle_timeout(Some(std::time::Duration::from_secs(600)))
        .connect_lazy(&db_url)
        .expect("Failed to create connection pool");

    DB_POOL.set(pool).expect("Failed to set DB_POOL");
}

/// ดึง Pool สำหรับใช้งาน
pub fn get_pool() -> &'static Pool<Postgres> {
    DB_POOL.get().expect("Database pool is not initialized")
}

/// ใช้ในการรัน Query แบบ Transaction
pub async fn query_task<F, T>(task: F) -> T
where
    F: FnOnce(&Pool<Postgres>) -> T,
{
    let pool = get_pool();
    task(pool)
}
