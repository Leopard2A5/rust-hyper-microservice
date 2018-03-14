extern crate hyper;
extern crate futures;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate dotenv;
#[macro_use] extern crate diesel;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate r2d2_diesel;

mod services;
mod db_connection;
mod schema;
mod models;

use hyper::server::Http;
use std::env;
use db_connection::build_connection_pool;

fn main() {
    env_logger::init();

    let pool = build_connection_pool();

    let addr = match env::var("BIND") {
        Ok(addr) => addr.parse().expect("Could not parse address and port"),
        Err(env::VarError::NotPresent) => "127.0.0.1:3000".parse().unwrap(),
        Err(env::VarError::NotUnicode(str)) => panic!("{:?} is not valid unicode!", str)
    };

    let server = Http::new()
        .bind(&addr, move || Ok(services::HelloService))
        .unwrap();

    info!("Server listening on {}", addr);
    server.run().unwrap();
}
