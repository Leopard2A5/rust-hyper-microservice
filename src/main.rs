extern crate hyper;
extern crate futures;
#[macro_use] extern crate log;
extern crate env_logger;

use hyper::server::Http;
mod services;

fn main() {
    env_logger::init();

    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new()
        .bind(&addr, || Ok(services::HelloService))
        .unwrap();

    info!("Server listening on {}", addr);
    server.run().unwrap();
}
