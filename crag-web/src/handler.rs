use crate::request::Request;
use crate::response;

pub trait Handler {
    fn handle(&self, request: Request) -> anyhow::Result<response::Response>;
}

// blanket implementation for all Fn that take a Request and return a Response
impl<F> Handler for F
where
    F: Fn(Request) -> anyhow::Result<response::Response> + Send + Sync + 'static,
{
    fn handle(&self, request: Request) -> anyhow::Result<response::Response> {
        self(request)
    }
}

pub type BoxedHandler = Box<dyn Handler + Send + Sync + 'static>;

/// Default handler for 404 errors
pub fn default_error_404_handler(_request: Request) -> anyhow::Result<response::Response> {
    Ok(response::Response::NotFound(
        include_bytes!("../static/html/404.html").to_owned().into(),
    ))
}
