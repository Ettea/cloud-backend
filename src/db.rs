// Placeholder for database connection
// Add SQLx configuration here when ready

use sqlx::PgPool;
use std::env;

pub async fn get_db_pool() -> Result<PgPool, sqlx::Error> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url).await
}

pub async fn init_db(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}
