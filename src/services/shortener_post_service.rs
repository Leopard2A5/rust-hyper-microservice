use hyper::{self, StatusCode};
use hyper::server::{Request, Response, Service};
use futures::future::{self, Future};
use ::request_utils::decode_query;
use diesel;
use diesel::prelude::*;
use db_connection::Connection;
use schema::urls::dsl::*;
use models::{Url, NewUrl};

pub struct ShortenerPostService(pub Connection);
impl Service for ShortenerPostService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let mut query = decode_query(req.query());
        let shrt = query.remove("short");
        let lng = query.remove("long");

        let response = if let (Some(shrt), Some(lng)) = (shrt, lng) {
            let url = NewUrl { short: &shrt, long: &lng };
            let result = diesel::insert_into(urls)
                .values(&url)
                .get_result::<Url>(&*self.0);
            match result {
                Ok(_) => Response::new(),
                Err(err) => {
                    error!("Error inserting into database: {}", err);
                    Response::new()
                        .with_status(StatusCode::InternalServerError)
                }
            }

        } else {
            Response::new()
                .with_status(StatusCode::BadRequest)
        };

        Box::new(future::ok(response))
    }
}
