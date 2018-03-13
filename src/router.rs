use hyper::{self, Method, StatusCode};
use hyper::server::{Request, Response, Service};
use futures::future::{self, Future};

use ::services::*;

pub struct Router;
impl Service for Router {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        info!("{} {}", req.method(), req.path());
        match (req.method(), req.path()) {
            (&Method::Get, "/") => HelloService{}.call(req),
            (&Method::Get, "/urls") => ShortenerGetService{}.call(req),
            _ => Box::new(
                future::ok(
                    Response::new()
                        .with_status(StatusCode::NotFound)
                )
            )
        }
    }
}
