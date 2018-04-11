extern crate hyper;
extern crate futures;
#[macro_use] extern crate log;
extern crate env_logger;
#[macro_use] extern crate maplit;
extern crate dotenv;
#[macro_use] extern crate diesel;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate r2d2_diesel;
extern crate regex;
#[macro_use] extern crate lazy_static;

mod services;
mod router;
mod request_utils;
mod db_connection;
mod schema;
mod models;

use hyper::server::Http;
use std::env;
use dotenv::dotenv;
use db_connection::build_connection_pool;

fn main() {
    dotenv().ok();

    env_logger::init();
    let _: std::collections::HashMap<&str, &str> = hashmap!{}; // get rid of unused import, maplit is used in tests

    let pool = build_connection_pool();

    let addr = match env::var("BIND") {
        Ok(addr) => addr.parse().expect("Could not parse address and port"),
        Err(env::VarError::NotPresent) => "127.0.0.1:3000".parse().unwrap(),
        Err(env::VarError::NotUnicode(str)) => panic!("{:?} is not valid unicode!", str)
    };

    let server = Http::new()
        .bind(&addr, move || Ok(router::Router(pool.clone())))
        .unwrap();

    info!("Server listening on {}", addr);
    server.run().unwrap();
}
