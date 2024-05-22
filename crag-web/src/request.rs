#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum Request {
    GET(String),
    POST(String, String),
}

impl Request {
    // should this be from implementation instead?
    pub fn parse(request_line: &String) -> anyhow::Result<Request> {
        println!("{request_line}");
        let mut parts = request_line.split_whitespace();

        let method = parts.next().unwrap_or("GET");
        let uri = parts.next().unwrap_or("not_implemented");
        let protocol = parts.next().unwrap_or("HTTP/1.1");

        if protocol != "HTTP/1.1" {
            panic!("Server can only work with HTTP/1.1");
        }

        let req = match method {
            "GET" => Request::GET(String::from(uri)),
            "POST" => Request::POST(String::from(uri), String::default()),
            _ => anyhow::bail!("Unrecognized method: {method}"),
        };

        Ok(req)
    }
    pub fn add_body(&mut self, body: String) {
        if let Request::POST(_, ref mut b) = self {
            *b = body;
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_parser_happy_path() {
        let req = Request::parse(&String::from("GET / HTTP/1.1")).unwrap();
        assert_eq!(req, Request::GET(String::from("/")));

        let req = Request::parse(&String::from("POST / HTTP/1.1")).unwrap();
        assert_eq!(req, Request::POST(String::from("/"), String::default()));
    }

    #[test]
    fn test_request_parser_bad_verbs() {
        let req = Request::parse(&String::from("FOO / HTTP/1.1"));
        assert!(req.is_err(), "Returned request is: {req:?}");
    }
}
