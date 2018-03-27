use hyper::StatusCode;
use hyper::server::{Request, Response};
use hyper::header::Location;
use ::request_utils::decode_query;
use ::services::{Service, Error};

use diesel::prelude::*;
use models::*;
use schema::urls::dsl;
use db_connection::Connection;

pub struct ShortenerGetService(pub Connection);
impl Service for ShortenerGetService {
    fn handle(&self, req: Request) -> Result<Response, Error> {
        let query = decode_query(req.query());
        let url = query
            .get("url")
            .ok_or(Error::ValidationError)?;

        let urls = dsl::urls
            .filter(dsl::short.eq(url))
            .load::<Url>(&*self.0)?;

        if let Some(url) = urls.get(0) {
            let text = url.long.clone();
            Ok(Response::new()
                .with_status(StatusCode::MovedPermanently)
                .with_header(Location::new(text)))
        } else {
            Ok(Response::new()
                .with_status(StatusCode::NotFound))
        }
    }
}
