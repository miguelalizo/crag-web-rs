use crate::request::Request;
use crate::response;

pub type Handler = Box<dyn Fn(Request) -> response::Response + Send + Sync + 'static>;

/// Default handler for 404 errors
pub fn default_error_404_handler(_request: Request) -> response::Response {
    response::Response::NotFound(include_str!("../static/html/404.html").to_owned())
}
