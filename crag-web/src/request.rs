// TODO: Add enumerated error values to not test based on strings

use anyhow::{bail, Result};

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum Request {
    GET(String),
    POST(String, String),
}

impl Request {
    // should this be from implementation instead?
    pub fn parse(request_line: &String) -> Result<Request> {
        println!("{request_line}");
        let mut parts = request_line.split_whitespace();

        let method = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("No method found"))?;

        let uri = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("No URI found"))?;

        let protocol = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("No protocol found"))?;

        if parts.next().is_some() {
            bail!("Invalid request line: extra values after parts");
        }

        if protocol != "HTTP/1.1" {
            bail!("Server can only work with HTTP/1.1");
        }

        let req = match method {
            "GET" => Request::GET(String::from(uri)),
            "POST" => Request::POST(String::from(uri), String::default()),
            _ => bail!("Unrecognized method: {method}"),
        };

        Ok(req)
    }

    pub fn add_body(&mut self, body: String) -> Result<(), anyhow::Error> {
        match self {
            Request::POST(_, ref mut b) => {
                if b.is_empty() {
                    *b = body;
                } else {
                    bail!("Body already exists in request")
                }
            }
            // TODO: Figure out if need to bail here?
            // is it valid for get reqs to ever have a body?
            _ => (),
        };
        Ok(())
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
    fn test_missing_verb() {
        let req = Request::parse(&String::from(""));
        assert!(req.is_err(), "Returned request is: {req:?}");
        assert!(req.err().unwrap().to_string().contains("No method found"));
    }

    #[test]
    fn test_request_parser_bad_verbs() {
        let req = Request::parse(&String::from("FOO / HTTP/1.1"));
        assert!(req.is_err(), "Returned request is: {req:?}");
        assert!(req
            .err()
            .unwrap()
            .to_string()
            .contains("Unrecognized method"));
    }

    #[test]
    fn test_missing_uri() {
        let req = Request::parse(&String::from("GET"));
        assert!(req.is_err(), "Returned request is: {req:?}");
        assert!(req.err().unwrap().to_string().contains("No URI found"));
    }

    #[test]
    fn test_missing_protocol() {
        let req = Request::parse(&String::from("GET /"));
        assert!(req.is_err(), "Returned request is: {req:?}");
        assert!(req.err().unwrap().to_string().contains("No protocol found"));
    }

    #[test]
    fn test_bad_protocol_name() {
        let req = Request::parse(&String::from("GET / HTTP/1.0"));
        assert!(req.is_err(), "Returned request is: {req:?}");
        assert!(req
            .err()
            .unwrap()
            .to_string()
            .contains("Server can only work with HTTP/1.1"));
    }

    #[test]
    fn test_good_paths() {
        let req = Request::parse(&String::from("GET / HTTP/1.1")).unwrap();
        assert_eq!(req, Request::GET(String::from("/")));

        let req = Request::parse(&String::from("GET /foo HTTP/1.1")).unwrap();
        assert_eq!(req, Request::GET(String::from("/foo")));

        let req = Request::parse(&String::from("GET /foo/bar HTTP/1.1")).unwrap();
        assert_eq!(req, Request::GET(String::from("/foo/bar")));
    }

    #[test]
    fn test_bad_missing_path() {
        let req = Request::parse(&String::from("GET"));
        assert!(req.is_err(), "Returned request is: {req:?}");
        assert!(req.err().unwrap().to_string().contains("No URI found"));
    }

    #[test]
    fn test_extra_content_in_request() {
        let req = Request::parse(&String::from("GET / HTTP/1.1 foo"));
        assert!(req.is_err(), "Returned request is: {req:?}");
        assert!(req
            .err()
            .unwrap()
            .to_string()
            .contains("Invalid request line: extra values after parts"));
    }

    #[test]
    fn test_add_body() {
        let mut req = Request::POST(String::from("/"), String::default());
        req.add_body(String::from("Hello, World!")).unwrap();
        assert_eq!(
            req,
            Request::POST(String::from("/"), String::from("Hello, World!"))
        );
    }

    #[test]
    fn test_add_body_twice() {
        let mut req = Request::POST(String::from("/"), String::default());
        req.add_body(String::from("Hello, World!")).unwrap();
        let res = req.add_body(String::from("Hello, World!"));
        assert!(res.is_err(), "Returned request is: {res:?}");
        assert!(res
            .err()
            .unwrap()
            .to_string()
            .contains("Body already exists in request"));
    }
}
