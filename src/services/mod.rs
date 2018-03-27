mod error;
mod hello_service;
mod shortener_get_service;
mod shortener_post_service;

pub use self::error::Error;
pub use self::hello_service::HelloService;
pub use self::shortener_get_service::ShortenerGetService;
pub use self::shortener_post_service::ShortenerPostService;

use hyper::{Request, Response};

pub trait Service {
    fn handle(
        &self,
        req: Request
    ) -> Result<Response, Error>;
}
