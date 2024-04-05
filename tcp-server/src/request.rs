
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

        let method = parts.next().unwrap_or("GET");
        let uri = parts.next().unwrap_or("not_implemented");
        let protocol = parts.next().unwrap_or("HTTP/1.1");

        if protocol != "HTTP/1.1" {
            panic!("Server can only work with HTTP/1.1");
        }

        match method {
            "GET" => return Request::GET(String::from(uri)),
            _ => return Request::UNIDENTIFIED
        }
    }
}


