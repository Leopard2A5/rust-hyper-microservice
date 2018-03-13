use hyper;
use hyper::server::{Request, Response, Service};
use hyper::header::ContentLength;
use futures::future::{self, Future};

pub struct HelloService;
impl Service for HelloService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, _req: Self::Request) -> Self::Future {
        let text = "Hello, world!";
        Box::new(
            future::ok(
                Response::new()
                    .with_header(ContentLength(text.len() as u64))
                    .with_body(text)
            )
        )
    }
}
