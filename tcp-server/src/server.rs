use std::collections::HashMap;
use std::net::{TcpListener, SocketAddr, TcpStream};
use std::io::{BufRead, Write};
use std::error;
use std::fmt;

use crate::threadpool::{self, PoolCreationError};
use crate::request;
use crate::handlers;
use crate::response;


#[derive(Debug)]
pub enum ServerError {
    ServerCreation(std::io::Error),
    PoolSizeError(PoolCreationError)
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for ServerError { }   



#[derive(Debug)]
pub struct Server {
    tcp_listener: TcpListener,
    pool: threadpool::ThreadPool,
    handlers: HashMap<request::Request, Box<dyn handlers::RequestHandler + Sync>>,
}

impl Server {
    pub fn build(
        socket_addr: SocketAddr,
        pool_size: usize,
        handlers: HashMap<request::Request, Box<dyn handlers::RequestHandler + Sync>>
    ) -> Result<Server, ServerError> {
       let tcp_listener = TcpListener::bind(socket_addr)
            .map_err(ServerError::ServerCreation)?;

        let pool = threadpool::ThreadPool::build(pool_size)
            .map_err(ServerError::PoolSizeError)?;

        let server = Server { 
            tcp_listener,
            pool,
            handlers,
        };

        Ok(server)

    }

    pub fn add_handler<T>(
        mut self,
        r: request::Request,
        handler: T,
    ) -> Self
    where T: handlers::RequestHandler + Sync + 'static
    {
        self.handlers.insert(r, Box::new(handler));
        self
    }

    pub fn run(&self) { 
        for stream in self.tcp_listener.incoming() {
            let stream = stream.unwrap(); // handle unwrap case later

            let cloned_handlers = self.handlers.clone();

            self.pool.execute( || {
                handle_connection(cloned_handlers, stream); //?
            });
        }

    }
}

fn handle_connection(server: &Server, mut stream: TcpStream){ //} -> std::io::Result<()> { propagate result to closure?
    // create buffer to store stream
    let mut buf = std::io::BufReader::new(&mut stream);

    // buffer to store request line (first line from buffer)
    let mut request_line = String::new();
    buf.read_line(&mut request_line).unwrap();

    // parse request
    let req = request::Request::build(request_line, server);

    // TODO Handle building Response from route handle functions
    let response = match server.handlers.get(&req) {
        Some(handler) => {
            handler.respond(buf)
        },
        None => {
            response::Response { content: String::from("404 NOT FOUUND") }
        }
    };

    // write response into TcpStream
    stream.write_all(response.content.as_bytes()).unwrap();//?;

}

