use hyper::server::{Request, Response};
use hyper::header::ContentLength;
use ::services::{Service, Error};

pub struct HelloService;
impl Service for HelloService {
    fn handle(&self, _req: Request) -> Result<Response, Error> {
        let text = "Hello, world!";
        Ok(Response::new()
            .with_header(ContentLength(text.len() as u64))
            .with_body(text))
    }
}
