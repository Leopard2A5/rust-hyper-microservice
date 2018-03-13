extern crate hyper;
extern crate futures;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate dotenv;
#[macro_use] extern crate diesel;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate r2d2_diesel;

use hyper::server::Http;
use std::env;
use dotenv::dotenv;
use diesel::prelude::*;
use diesel::pg::PgConnection;

mod services;
mod schema;
mod models;

fn build_connection_pool() -> r2d2::Pool<r2d2_diesel::ConnectionManager<PgConnection>> {
    dotenv().ok();
    let url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set!");
    let manager = r2d2_diesel::ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("Failed to create pool")
}

fn main() {
    env_logger::init();

    let pool = build_connection_pool();

    let addr = match env::var("BIND") {
        Ok(addr) => addr.parse().expect("Could not parse address and port"),
        Err(env::VarError::NotPresent) => "127.0.0.1:3000".parse().unwrap(),
        Err(env::VarError::NotUnicode(str)) => panic!("{:?} is not valid unicode!", str)
    };

    let server = Http::new()
        .bind(&addr, move || Ok(services::HelloService(pool.clone())))
        .unwrap();

    info!("Server listening on {}", addr);
    server.run().unwrap();
}
