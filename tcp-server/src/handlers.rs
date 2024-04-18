use std::net::TcpStream;
use std::fmt;

use crate::response::Response;


pub trait RequestHandler: fmt::Debug {
    fn respond(&self, buf: std::io::BufReader<&mut TcpStream>) -> Response;
}

