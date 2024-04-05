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
    pub fn build(request_vec: Vec<String>) -> Request {
        let header_vec: Vec<&str> = request_vec
            .get(0)
            .unwrap()
            .split(" ")
            .collect();
        
        let method = *header_vec.get(0).unwrap();
        let uri = *header_vec.get(1).unwrap();
        let protocol = *header_vec.get(2).unwrap();

        println!("{}, {}, {}", method, uri, protocol);
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


