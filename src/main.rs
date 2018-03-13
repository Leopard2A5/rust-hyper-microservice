extern crate hyper;
extern crate futures;
#[macro_use] extern crate log;
extern crate env_logger;
#[macro_use] extern crate maplit;

use hyper::server::Http;
use std::env;

mod services;
mod router;
mod request_utils;

fn main() {
    env_logger::init();
    let _: std::collections::HashMap<&str, &str> = hashmap!{}; // get rid of unused import, maplit is used in tests

    let addr = match env::var("BIND") {
        Ok(addr) => addr.parse().expect("Could not parse address and port"),
        Err(env::VarError::NotPresent) => "127.0.0.1:3000".parse().unwrap(),
        Err(env::VarError::NotUnicode(str)) => panic!("{:?} is not valid unicode!", str)
    };

    let server = Http::new()
        .bind(&addr, || Ok(router::Router))
        .unwrap();

    info!("Server listening on {}", addr);
    server.run().unwrap();
}
