use hyper::StatusCode;
use hyper::server::{Request, Response};
use hyper::header::Location;
use ::request_utils::decode_query;
use ::services::{Service, Error};

use diesel::prelude::*;
use models::*;
use schema::urls::dsl;
use db_connection::Connection;

use regex::Regex;

lazy_static!{
    static ref RE: Regex = Regex::new(r"/urls/(.+)").unwrap();
}

pub struct ShortenerGetService(pub Connection);
impl Service for ShortenerGetService {
    fn handle(&self, req: Request) -> Result<Response, Error> {
        let url = RE.captures(req.path())
            .ok_or(Error::MissingPathParam)?
            .get(1).unwrap().as_str();

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
