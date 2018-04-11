use hyper::{self, Method};
use hyper::StatusCode::{BadRequest, InternalServerError, NotFound};
use hyper::server::{Request, Response};
use futures::future::{self, Future};

use ::services::*;
use ::services::Service;
use db_connection::ConnectionPool;

pub struct Router(pub ConnectionPool);
impl hyper::server::Service for Router {
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
                        .with_status(InternalServerError)
                ))
            }
        };

        let result = if *req.method() == Method::Get && req.path() == "/" {
            HelloService{}.handle(req)
        } else if *req.method() == Method::Get && req.path().starts_with("/urls") {
            ShortenerGetService(conn).handle(req)
        } else if *req.method() == Method::Post && req.path() == "/urls" {
            ShortenerPostService(conn).handle(req)
        } else {
            Ok(Response::new().with_status(NotFound))
        };

        let response = match result {
            Ok(response) => response,
            Err(Error::ValidationError) => Response::new().with_status(BadRequest),
            Err(Error::InternalServerError) => Response::new().with_status(InternalServerError),
            Err(Error::MissingPathParam) => Response::new().with_status(NotFound),
        };

        Box::new(future::ok(response))
    }
}
