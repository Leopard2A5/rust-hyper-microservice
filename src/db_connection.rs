use std::env;
use dotenv::dotenv;
use diesel::pg::PgConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;

pub type ConnectionPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn build_connection_pool() -> ConnectionPool {
    dotenv().ok();

    let url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set!");

    let manager = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("Failed to create pool")
}
