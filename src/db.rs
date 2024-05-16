use sqlx::SqlitePool;

pub async fn init_pool() -> SqlitePool {
    let pool = SqlitePool::connect_lazy("sqlite::memory:").unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();
    pool
}