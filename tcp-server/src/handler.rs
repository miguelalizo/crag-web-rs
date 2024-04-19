use crate::response;

pub type Handler = fn() -> response::Response;