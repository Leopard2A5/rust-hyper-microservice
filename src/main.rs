extern crate hyper;
extern crate futures;
#[macro_use] extern crate log;
extern crate env_logger;

use hyper::server::Http;
use std::env;

mod services;

fn main() {
    env_logger::init();

    let addr = match env::var("BIND") {
        Ok(addr) => addr.parse().expect("Could not parse address and port"),
        Err(env::VarError::NotPresent) => "127.0.0.1:3000".parse().unwrap(),
        Err(env::VarError::NotUnicode(str)) => panic!("{:?} is not valid unicode!", str)
    };

    let server = Http::new()
        .bind(&addr, || Ok(services::HelloService))
        .unwrap();

    info!("Server listening on {}", addr);
    server.run().unwrap();
}
