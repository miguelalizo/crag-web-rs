use crate::methods::Method;
use crate::routes::Route;

use anyhow::{bail, Result};

// TODO: Add enumerated error values to not test based on strings

#[derive(Eq, PartialEq, Debug)]
pub struct Request {
    pub method: Method,
    pub route: Route,
    pub body: Option<String>,
}

impl Request {
    pub fn new(method: Method, route: Route) -> Self {
        Request {
            method,
            route,
            body: None,
        }
    }

    // should this be from implementation instead?
    pub fn parse(request_line: impl AsRef<str>) -> Result<Request> {
        let request_line = request_line.as_ref();
        println!("{request_line}");

        let mut parts = request_line.split_whitespace();

        let method = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("No method found"))?;

        let route = parts
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

        let method = match method {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => bail!("Unrecognized method: {method}"),
        };

        Ok(Request::new(method, route.into()))
    }

    pub fn add_body(&mut self, body: String) -> Result<(), anyhow::Error> {
        if let &mut Method::POST = &mut self.method {
            if self.body.is_none() {
                self.body = Some(body);
            } else {
                bail!("Body already exists in request")
            }
            // TODO: Figure out if need to bail here?
            // is it valid for get reqs to ever have a body?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_parser_happy_path() {
        let req = Request::parse(&String::from("GET / HTTP/1.1")).unwrap();
        assert_eq!(req.method, Method::GET);
        assert_eq!(req.route, "/".into(),);
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
        assert_eq!(req.method, Method::GET);
        assert_eq!(req.route, "/".into(),);

        let req = Request::parse(&String::from("GET /foo HTTP/1.1")).unwrap();
        assert_eq!(req.method, Method::GET);
        assert_eq!(req.route, "/foo".into());

        let req = Request::parse(&String::from("GET /foo/bar HTTP/1.1")).unwrap();
        assert_eq!(req.method, Method::GET);
        assert_eq!(req.route, "/foo/bar".into());
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
    fn test_add_body_to_get_request() {
        let mut req = Request::new(Method::GET, "/".into());
        let res = req.add_body(String::from("Hello, World!"));
        assert!(res.is_ok());
        assert!(req.body.is_none());
    }

    #[test]
    fn test_add_body_to_post_request() {
        let mut req = Request::new(Method::POST, "/".into());
        req.add_body(String::from("Hello, World!")).unwrap();
        assert_eq!(req.body, Some(String::from("Hello, World!")));
    }

    #[test]
    fn test_add_body_twice() {
        let mut req = Request::new(Method::POST, "/".into());
        req.add_body(String::from("Hello, World!")).unwrap();
        let res = req.add_body(String::from("Hello, World!"));
        assert!(res
            .err()
            .unwrap()
            .to_string()
            .contains("Body already exists in request"));
    }
}
