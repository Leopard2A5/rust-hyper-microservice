use hyper::server::{Request, Response};
use ::request_utils::decode_query;
use diesel;
use diesel::prelude::*;
use db_connection::Connection;
use schema::urls::dsl::*;
use models::{Url, NewUrl};
use ::services::{Service, Error};

pub struct ShortenerPostService(pub Connection);
impl Service for ShortenerPostService {
    fn handle(&self, req: Request) -> Result<Response, Error> {
        let mut query = decode_query(req.query());
        let shrt = query
            .remove("short")
            .ok_or(Error::ValidationError)?;
        let lng = query
            .remove("long")
            .ok_or(Error::ValidationError)?;

        let url = NewUrl { short: &shrt, long: &lng };
        diesel::insert_into(urls)
            .values(&url)
            .get_result::<Url>(&*self.0)?;

        Ok(Response::new())
    }
}
