use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::ToSocketAddrs;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use tracing::error;

use crate::handler;
use crate::request;
use crate::response;
use crate::threadpool;

pub struct Server {
    tcp_listener: TcpListener,
    pool: threadpool::ThreadPool,
    handlers: Arc<HashMap<request::Request, handler::Handler>>,
}

pub struct ServerBuilder {
    handlers: HashMap<request::Request, handler::Handler>,
}

impl ServerBuilder {
    pub fn finalize(self, addr: impl ToSocketAddrs, pool_size: usize) -> Result<Server> {
        let socket_addr = addr
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| anyhow::anyhow!("Unable to resolve address"))?;

        let tcp_listener = TcpListener::bind(socket_addr)?;

        let pool = threadpool::ThreadPool::build(pool_size)?;

        let server = Server {
            tcp_listener,
            pool,
            handlers: Arc::new(self.handlers),
        };

        Ok(server)
    }

    pub fn register_handler(
        mut self,
        r: request::Request,
        handler: impl Fn(request::Request) -> response::Response + Send + Sync + 'static,
    ) -> Self {
        self.handlers.insert(r, Box::new(handler));
        self
    }

    pub fn register_error_handler(
        self,
        handler: impl Fn(request::Request) -> response::Response + Send + Sync + 'static,
    ) -> Self {
        let request = request::Request::UNIDENTIFIED;
        self.register_handler(request, handler)
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
                if let Err(e) = handle_connection(handlers, &mut stream) {
                    error!("Error handling connection: {:?}", e);
                };
            });
        }
        Ok(())
    }
}

fn handle_connection(
    handlers: Arc<HashMap<request::Request, handler::Handler>>,
    stream: &mut TcpStream,
) -> Result<()> {
    let req = parse_request(stream).map_err(|err| anyhow!("Error parsing request: {:?}", err))?;
    let hashed_req = match req {
        request::Request::GET(ref a) => request::Request::GET(a.clone()),
        request::Request::POST(ref a, _) => request::Request::POST(a.clone(), String::default()),
        request::Request::UNIDENTIFIED => request::Request::UNIDENTIFIED,
    };

    // build response
    let response = match handlers.get(&hashed_req) {
        Some(handler) => handler(req),
        None => match handlers.get(&request::Request::UNIDENTIFIED) {
            Some(handler) => handler(req),
            None => handler::default_error_404_handler(req),
        },
    };

    // write response into TcpStream
    stream.write_all(&Vec::<u8>::from(response))?;

    stream.shutdown(std::net::Shutdown::Both)?;

    Ok(())
}

// TODO: Fix return type
fn parse_request(stream: &mut TcpStream) -> Result<request::Request> {
    // create buffer
    let mut request: Vec<String> = vec![];
    let mut buffer = BufReader::new(stream);

    // Read the HTTP request headers until end of header
    while request.is_empty() || request.last().insert(&String::default()).len() > 2 {
        let mut next_line = String::new();
        buffer.read_line(&mut next_line)?;
        request.push(next_line);
    }

    // build request from header
    let mut req = request::Request::build(request.first().unwrap_or(&"/".to_owned()).to_owned());

    if let request::Request::POST(_, _) = req {
        // Find the Content-Length header
        let content_length = request
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

        // Parse the request body based on Content-Length
        // TODO: Ask John about read_to_end vs read
        // Read to end blocks until the client closes the connection
        // which it will not until the server sends a response
        // thus it will block until client times out
        let mut body_buffer = vec![0; content_length];
        buffer.read_exact(&mut body_buffer)?;

        // Add body to request
        req.add_body(String::from_utf8(body_buffer.clone()).unwrap_or_default());
    };

    Ok(req)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::handler;
    use crate::request;
    use crate::response;

    // get "/hello"
    fn hello_handler(_request: request::Request) -> response::Response {
        response::Response::Ok("Hello, Crag-Web!".to_owned())
    }

    #[test]
    fn test_builder_pattern() {
        // Create server
        let _builder = Server::build()
            .register_error_handler(Box::new(handler::default_error_404_handler))
            .register_handler(
                request::Request::GET("/".to_owned()),
                Box::new(|_req| response::Response::Ok("Hello, Crag-Web!".to_owned())),
            )
            .register_handler(
                request::Request::GET("/hello".to_owned()),
                Box::new(hello_handler),
            )
            .finalize(("127.0.0.1", 8010), 4)
            .unwrap();
    }
}
