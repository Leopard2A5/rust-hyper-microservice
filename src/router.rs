use hyper::{self, Method, StatusCode};
use hyper::server::{Request, Response, Service};
use futures::future::{self, Future};

use ::services::*;
use db_connection::ConnectionPool;

pub struct Router(pub ConnectionPool);
impl Service for Router {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        info!("{} {}", req.method(), req.path());
        let conn = self.0.get();
        let conn = match conn {
            Ok(c) => c,
            Err(err) => {
                error!("Error connecting to database: {:?}", err);
                return Box::new(future::ok(
                    Response::new()
                        .with_status(StatusCode::InternalServerError)
                ))
            }
        };

        match (req.method(), req.path()) {
            (&Method::Get,  "/") => HelloService{}.call(req),
            (&Method::Get,  "/urls") => ShortenerGetService(conn).call(req),
            (&Method::Post, "/urls") => ShortenerPostService(conn).call(req),
            _ => Box::new(
                future::ok(
                    Response::new()
                        .with_status(StatusCode::NotFound)
                )
            )
        }
    }
}
