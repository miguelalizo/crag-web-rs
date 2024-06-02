use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use tracing::error;

use crate::handler;
use crate::request;
use crate::response;
use crate::threadpool;

type HandlerMap = HashMap<request::Request, handler::BoxedHandler>;

struct Handlers {
    valid_handlers: HandlerMap,
    error_handler: handler::BoxedHandler,
}

impl Handlers {
    fn handle_error(&self, req: request::Request) -> Result<response::Response> {
        self.error_handler.handle(req)
    }
}

pub struct Server {
    tcp_listener: TcpListener,
    pool: threadpool::ThreadPool,
    handlers: Arc<Handlers>,
}

pub struct ServerBuilder {
    handlers: HandlerMap,
    error_handler: Option<handler::BoxedHandler>,
}

impl ServerBuilder {
    /// Finalize the server builder and create a server instance
    /// an error handler must always be defined or this will err.
    pub fn finalize(self, addr: impl ToSocketAddrs, pool_size: usize) -> Result<Server> {
        // Check to see that there is an error_handler for 404 errors
        let error_handler = match self.error_handler {
            Some(handler) => handler,
            None => anyhow::bail!("Error: No error handler defined"),
        };

        let socket_addr = addr
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| anyhow::anyhow!("Unable to resolve address"))?;

        let tcp_listener = TcpListener::bind(socket_addr)?;
        let pool = threadpool::ThreadPool::build(pool_size)?;
        let handlers = Arc::new(Handlers {
            valid_handlers: self.handlers,
            error_handler,
        });

        let server = Server {
            tcp_listener,
            pool,
            handlers,
        };

        Ok(server)
    }

    pub fn register_handler(
        mut self,
        r: request::Request,
        handler: impl handler::Handler + Send + Sync + 'static,
    ) -> Result<Self> {
        if self.handlers.contains_key(&r) {
            anyhow::bail!("Handler already registered for {r:?}");
        }
        self.handlers.insert(r, Box::new(handler));
        Ok(self)
    }

    pub fn register_error_handler(
        mut self,
        handler: impl handler::Handler + Send + Sync + 'static,
    ) -> Result<Self> {
        if self.error_handler.is_some() {
            anyhow::bail!("Error handler already registered");
        }
        self.error_handler = Some(Box::new(handler));
        Ok(self)
    }
}

impl Server {
    pub fn build() -> ServerBuilder {
        ServerBuilder {
            handlers: HashMap::new(),
            error_handler: None,
        }
    }
    pub fn run(&self) -> Result<()> {
        for stream in self.tcp_listener.incoming() {
            let mut stream = stream?;
            let handlers = self.handlers.clone();

            // error boundary
            // does trying to return 404 or 501 on error make sense when the error coming from
            self.pool.execute(move || {
                if let Err(e) = handle_connection(&handlers, &mut stream) {
                    error!("Error handling connection: {:?}", e);
                    _ = stream.write_all("HTTP/1.1 501 Internal Server Error\r\n\r\n".as_bytes());
                };
            });
        }
        Ok(())
    }
}

fn handle_connection<S>(handlers: &Handlers, stream: &mut S) -> Result<()>
where
    S: Read + Write,
{
    let req = read_and_parse_request(stream)
        .map_err(|err| anyhow!("Error parsing request: {:?}", err))?;

    // build response
    let response = match handlers.valid_handlers.get(&req) {
        Some(handler) => handler.handle(req),
        None => handlers.handle_error(req),
    };
    let response = response?;

    // write response into TcpStream
    stream.write_all(&Vec::<u8>::from(response))?;

    Ok(())
}

fn read_and_parse_request(stream: &mut impl Read) -> Result<request::Request> {
    // create buffer
    let mut buffer = BufReader::new(stream);

    // get header lines
    let lines = {
        let mut lines: Vec<String> = vec![];
        loop {
            let mut next_line = String::new();
            buffer.read_line(&mut next_line)?;
            if next_line.is_empty() || next_line == "\r\n" || next_line == "\r" {
                break lines;
            }
            lines.push(next_line);
        }
    };

    // Parse the request and content_length for body
    let (mut req, content_length) = parse_request(lines)?;

    // Parse the request body based on Content-Length
    let mut body_buffer = vec![0; content_length];
    buffer.read_exact(&mut body_buffer)?;

    // Add body to request if POST
    if let request::Request::POST(_, _) = req {
        if content_length > 0 {
            req.add_body(String::from_utf8(body_buffer.clone()).unwrap_or_default())?;
        }
    }

    Ok(req)
}

fn parse_request<IT, S>(lines: IT) -> Result<(request::Request, usize)>
where
    IT: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut lines = lines.into_iter();
    // build request from header
    let first_line = lines
        .next()
        .ok_or_else(|| anyhow!("No request line found"))?;
    let req = request::Request::parse(first_line.as_ref())?;

    // parse content length if POST else 0
    let content_length = match req {
        request::Request::GET(_) => 0,
        request::Request::POST(_, _) => {
            lines
                .find(|line| line.as_ref().starts_with("Content-Length:"))
                .and_then(|line| {
                    line.as_ref()
                        .trim()
                        .split(':')
                        .nth(1)
                        .and_then(|value| value.trim().parse::<usize>().ok())
                })
                .unwrap_or(0)
            // panic!("Need to read body according to content length but we are not doing that yet")
        }
    };

    Ok((req, content_length))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::handler;
    use crate::request;
    use crate::response;
    use anyhow::Result;

    // get "/hello"
    fn hello_handler(_request: request::Request) -> Result<response::Response> {
        Ok(response::Response::Ok(
            "Hello, Crag-Web!".as_bytes().to_vec(),
            response::ContentType::HTML,
        ))
    }

    #[test]
    fn test_builder_pattern() -> Result<()> {
        // Create server
        let _builder = Server::build()
            .register_error_handler(Box::new(handler::default_error_404_handler))?
            .register_handler(
                request::Request::GET("/".to_owned()),
                Box::new(|_req| {
                    Ok(response::Response::Ok(
                        "Hello, Crag-Web!".as_bytes().to_vec(),
                        response::ContentType::HTML,
                    ))
                }),
            )?
            .register_handler(
                request::Request::GET("/hello".to_owned()),
                Box::new(hello_handler),
            )?
            .finalize(("127.0.0.1", 8010), 4)
            .unwrap();
        Ok(())
    }

    #[test]
    fn test_no_err_handler_fails() -> Result<()> {
        let server = Server::build()
            .register_handler(
                request::Request::GET("/".to_owned()),
                Box::new(|_req| {
                    Ok(response::Response::Ok(
                        "Hello, Crag-Web!".as_bytes().to_vec(),
                        response::ContentType::HTML,
                    ))
                }),
            )?
            .finalize(("127.0.0.1", 8011), 1);
        assert!(server.is_err());
        Ok(())
    }

    #[test]
    fn test_parse_request_get() -> Result<()> {
        let lines = &["GET / HTTP/1.1"];
        let (req, content_length) = parse_request(lines.iter())?;
        assert_eq!(req, request::Request::GET("/".to_owned()));
        assert_eq!(content_length, 0);

        Ok(())
    }

    #[test]
    fn test_parse_request_post() -> Result<()> {
        let lines = &["POST / HTTP/1.1", "Content-Length: 0"];
        let (req, content_length) = parse_request(lines.iter())?;
        assert_eq!(
            req,
            request::Request::POST("/".to_owned(), String::default())
        );
        assert_eq!(content_length, 0);

        let lines = &["POST / HTTP/1.1", "Content-Length: 10", "foobarfoob"];
        let (req, content_length) = parse_request(lines.iter())?;
        assert_eq!(
            req,
            request::Request::POST("/".to_owned(), String::from(""))
        );
        assert_eq!(content_length, 10);

        Ok(())
    }

    #[test]
    fn test_parse_request_empty() -> Result<()> {
        let empty: &[&str; 0] = &[];
        let res = parse_request(empty.iter());
        assert!(res.is_err());
        assert_eq!(res.err().unwrap().to_string(), "No request line found");
        Ok(())
    }

    #[test]
    fn test_read_and_parse_request_get() -> Result<()> {
        let req = vec![
            "GET / HTTP/1.1\r\n",
            "Content-Length: 13\r\n",
            "\r\n",
            "Hello, World!",
        ]
        .join("");
        let mut stream = req.as_bytes();

        // turn stream into BufReader
        let res = read_and_parse_request(&mut stream)?;
        assert_eq!(res, request::Request::GET("/".to_owned()));
        Ok(())
    }

    #[test]
    fn test_read_and_parse_request_post() -> Result<()> {
        let req = vec![
            "POST / HTTP/1.1\r\n",
            "Content-Length: 13\r\n",
            "\r\n",
            "Hello, World!",
        ]
        .join("");
        let mut stream = req.as_bytes();

        // turn stream into BufReader
        let res = read_and_parse_request(&mut stream)?;
        assert_eq!(
            res,
            request::Request::POST("/".to_owned(), "Hello, World!".to_owned())
        );
        Ok(())
    }

    // #[test]
}
