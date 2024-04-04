use std::net::{TcpListener, SocketAddr, TcpStream};
use std::io::{BufRead, Write};
use std::error;
use std::fmt;

use crate::threadpool::{self, PoolCreationError};


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


pub struct Server {
    tcp_listener: TcpListener,
    pool: threadpool::ThreadPool,
}

impl Server {
    pub fn new(socket_addr: SocketAddr, pool_size: usize) -> Result<Server, ServerError> {
       let tcp_listener = TcpListener::bind(socket_addr)
            .map_err(ServerError::ServerCreation)?;

        let pool = threadpool::ThreadPool::build(pool_size)
            .map_err(ServerError::PoolSizeError)?;
    
        let server = Server { 
            tcp_listener,
            pool
        };

        Ok(server)

    }
    pub fn run(&self) { 
        for stream in self.tcp_listener.incoming() {
            let stream = stream.unwrap(); // handle unwrap case later

            self.pool.execute( || {
                handle_connection(stream); //?
            });
        }

    }

}


fn handle_connection(mut stream: TcpStream){ //} -> std::io::Result<()> { 
    // create buffer to store stream lines   
    let buf = std::io::BufReader::new(&mut stream);

    // parse request
    // TODO: create Request struct
    let request_line = buf
        .lines()
        .next()
        .unwrap().unwrap();//?;
       
    // serve a response based on the request line
    // TODO: handle this with route handler later
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "../static/html/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "../static/html/404.html")
    };

    // read html file contents into a String
    // and get len
    let html_contents = std::fs::read_to_string(filename).unwrap();//?;
    let len = html_contents.len();

    // format http response
    let response = format!(
        "{status_line}\r\nContent-Length: {len}\r\n\r\n{html_contents}"
    );

    // write response into TcpStream
    stream
        .write_all(response.as_bytes()).unwrap();//?;

    
    // Ok(())

}