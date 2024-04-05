use std::io::BufReader;
use std::net:: TcpStream;
use std::io::BufRead;

#[derive(Debug)]
pub enum Request {
    GET(String),
    UNIDENTIFIED
}

impl Request {
    // should this be from instead?
    pub fn build(request_line: String) -> Request {
        let mut parts = request_line
            .trim()
            .split_whitespace();

        let method = parts.next().unwrap();
        let uri = parts.next().unwrap();
        let protocol = parts.next().unwrap();

        if protocol != "HTTP/1.1" {
            panic!("Server can only work with HTTP/1.1");
        }

        if method == "GET" {
            return Request::GET(String::from(uri))
        } else {
            return Request::UNIDENTIFIED
        }
    }
}


