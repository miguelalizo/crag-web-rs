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

type HandlerMap = HashMap<Option<request::Request>, handler::Handler>;

pub struct Server {
    tcp_listener: TcpListener,
    pool: threadpool::ThreadPool,
    handlers: Arc<HandlerMap>,
}

pub struct ServerBuilder {
    handlers: HandlerMap,
}

impl ServerBuilder {
    pub fn finalize(self, addr: impl ToSocketAddrs, pool_size: usize) -> Result<Server> {
        let socket_addr = addr
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| anyhow::anyhow!("Unable to resolve address"))?;

        let tcp_listener = TcpListener::bind(socket_addr)?;
        let pool = threadpool::ThreadPool::build(pool_size)?;
        let handlers = Arc::new(self.handlers);

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
        handler: impl Fn(request::Request) -> Result<response::Response> + Send + Sync + 'static,
    ) -> Self {
        // TODO: Communicate to user when existing key overwritten
        self.handlers.insert(Some(r), Box::new(handler));
        self
    }

    pub fn register_error_handler(
        mut self,
        handler: impl Fn(request::Request) -> Result<response::Response> + Send + Sync + 'static,
    ) -> Self {
        // TODO: Communicate to user when existing key overwritten
        self.handlers.insert(None, Box::new(handler));
        self
    }
}

impl Server {
    pub fn build() -> ServerBuilder {
        ServerBuilder {
            handlers: HashMap::new(),
        }
    }
    pub fn run(&self) -> Result<()> {
        for stream in self.tcp_listener.incoming() {
            let mut stream = stream?;
            let handlers = self.handlers.clone();

            // error boundary
            // does trying to return 404 or 501 on error make sense when the error coming from
            // handle_conn could be on stream.shutdown?
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

fn handle_connection<S>(handlers: &HandlerMap, stream: &mut S) -> Result<()>
where
    S: Read + Write,
{
    let req =
        read_parse_request(stream).map_err(|err| anyhow!("Error parsing request: {:?}", err))?;

    // build response
    let response = match handlers.get(&Some(req.clone())) {
        Some(handler) => handler(req),
        None => {
            if let Some(four_oh_four_handler) = handlers.get(&None) {
                four_oh_four_handler(req)
            } else {
                handler::default_error_404_handler(req)
            }
        }
    };
    let response = response?;

    // write response into TcpStream
    stream.write_all(&Vec::<u8>::from(response))?;

    Ok(())
}

fn read_parse_request(stream: &mut impl Read) -> Result<request::Request> {
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
    let (mut req, content_length) = parse_request(&lines)?;

    // Parse the request body based on Content-Length
    let mut body_buffer = vec![0; content_length];
    buffer.read_exact(&mut body_buffer)?;

    // Add body to request
    req.add_body(String::from_utf8(body_buffer.clone()).unwrap_or_default());

    Ok(req)
}

fn parse_request(lines: &[String]) -> Result<(request::Request, usize)> {
    // build request from header
    let first_line = lines
        .first()
        .ok_or_else(|| anyhow!("No request line found"))?;
    let req = request::Request::parse(first_line)?;
    let content_length = match req {
        request::Request::GET(_) => 0,
        request::Request::POST(_, _) => {
            lines
                .iter()
                // .lines()
                .find(|line| line.starts_with("Content-Length:"))
                .and_then(|line| {
                    line.trim()
                        .split(':')
                        .nth(1)
                        .and_then(|value| value.trim().parse::<usize>().ok())
                })
                .unwrap_or(0);
            panic!("Need to read body according to content length but we are not doing that yet")
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
        Ok(response::Response::Ok("Hello, Crag-Web!".to_owned()))
    }

    #[test]
    fn test_builder_pattern() {
        // Create server
        let _builder = Server::build()
            .register_error_handler(Box::new(handler::default_error_404_handler))
            .register_handler(
                request::Request::GET("/".to_owned()),
                Box::new(|_req| Ok(response::Response::Ok("Hello, Crag-Web!".to_owned()))),
            )
            .register_handler(
                request::Request::GET("/hello".to_owned()),
                Box::new(hello_handler),
            )
            .finalize(("127.0.0.1", 8010), 4)
            .unwrap();
    }
}
