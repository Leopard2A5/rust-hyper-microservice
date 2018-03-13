use hyper;
use hyper::server::{Request, Response, Service};
use hyper::header::ContentLength;
use futures::future::{self, Future};
use ::schema::urls::dsl::*;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use r2d2;
use diesel::PgConnection;
use r2d2_diesel::ConnectionManager;

pub struct HelloService(pub r2d2::Pool<ConnectionManager<PgConnection>>);
impl Service for HelloService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, _req: Self::Request) -> Self::Future {
        let conn = self.0.get().unwrap();
        let count = urls
            .count()
            .load::<i64>(&*conn)
            .expect("Error querying database!");
        let count = count.first()
            .unwrap();
        let text = format!("Hello, world! {}", count);

        Box::new(
            future::ok(
                Response::new()
                    .with_header(ContentLength(text.len() as u64))
                    .with_body(text)
            )
        )
    }
}
