use hyper::{self, StatusCode};
use hyper::server::{Request, Response, Service};
use hyper::header::Location;
use futures::future::{self, Future};
use ::request_utils::decode_query;

use diesel::prelude::*;
use models::*;
use schema::urls::dsl::*;
use db_connection::Connection;

pub struct ShortenerGetService(pub Connection);
impl Service for ShortenerGetService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let query = decode_query(req.query());
        let url = query.get("url");

        let response = match url {
            None => Response::new()
                .with_status(StatusCode::BadRequest),
            Some(url) => {
                let tmp = urls
                    .filter(short.eq(url))
                    .load::<Url>(&*self.0);
                match tmp {
                    Ok(vec) => {
                        if let Some(url) = vec.get(0) {
                            let text = url.long.clone();
                            Response::new()
                                .with_status(StatusCode::MovedPermanently)
                                .with_header(Location::new(text))
                        } else {
                            Response::new()
                                .with_status(StatusCode::NotFound)
                        }
                    },
                    Err(err) => {
                        error!("Error querying database: {}", err);
                        Response::new()
                            .with_status(StatusCode::InternalServerError)
                    }
                }
            }
        };
        Box::new(future::ok(response))
    }
}
