use hyper::{self, StatusCode};
use hyper::server::{Request, Response, Service};
use hyper::header::{ContentLength, Location};
use futures::future::{self, Future};
use ::request_utils::decode_query;

pub struct ShortenerGetService;
impl Service for ShortenerGetService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let query = decode_query(req.query());
        let url = query.get("url");
        let value = url
            .map(|_url| "http://www.google.com");

        Box::new(future::ok(
            match value {
                Some(val) => Response::new()
                    .with_status(StatusCode::MovedPermanently)
                    .with_header(Location::new(val))
                ,
                None => Response::new()
                    .with_header(ContentLength(0))
            }
        ))
    }
}
