use crate::request::Request;
use crate::response::{self, IntoBytes};

pub type Handler<T: IntoBytes> =
    Box<dyn Fn(Request) -> anyhow::Result<response::Response<T>> + Send + Sync + 'static>;

/// Default handler for 404 errors
pub fn default_error_404_handler(_request: Request) -> anyhow::Result<response::Response<Vec<u8>>> {
    Ok(response::Response::NotFound(
        include_bytes!("../static/html/404.html").to_owned().into(),
    ))
}
