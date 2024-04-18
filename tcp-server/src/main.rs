use std::net::{TcpStream, ToSocketAddrs};

mod threadpool;
mod server;
mod request;
mod handlers;
mod response;

#[derive(Debug, Clone)]
struct NotFoundHandler {}

impl handlers::RequestHandler for NotFoundHandler {
    fn respond(&self, buf: std::io::BufReader<&mut TcpStream>) -> response::Response {
        response::Response { content: String::from("content") }
    }
}

#[derive(Debug, Clone)]
struct GetIndexHandler{}

impl handlers::RequestHandler for GetIndexHandler {
    // fulfil request -> Response
    fn respond(&self, buf: std::io::BufReader<&mut TcpStream>) -> response::Response {
        // add HTTP header
        // add the right html file
        // make into json?
        // return Response
        response::Response { content: String::from("content") }
    }
}


#[derive(Debug, Clone)]
struct GetContactHandler{}

impl handlers::RequestHandler for GetContactHandler {
    // fulfil request -> Response
    fn respond(&self, buf: std::io::BufReader<&mut TcpStream>) -> response::Response {
        // add HTTP header
        // add the right html file
        // make into json?
        // return GetThankYouHandler::respond()
        response::Response { content: String::from("content") }
    }
}

#[derive(Debug, Clone)]
struct GetThankYouHandler {}

impl handlers::RequestHandler for GetThankYouHandler {
    // fulfil request -> Response
    fn respond(&self, buf: std::io::BufReader<&mut TcpStream>) -> response::Response {
        // add HTTP header
        // add the right html file
        // make into json?
        // return Response
        response::Response { content: String::from("content") }
    }
}


#[derive(Debug, Clone)]
struct PostContactHandler {}

impl handlers::RequestHandler for PostContactHandler {
    fn respond(&self, buf: std::io::BufReader<&mut TcpStream>) -> response::Response {
    // fulfil request -> Response
        // parse body
        // send email
        // return Response with "thank you" page

    // add_body()
        response::Response { content: String::from("content") }
    }
}



fn main() -> std::io::Result<()> {
    // validate addr
    let addr = "127.0.0.1:8010";
    let socket_addr = match addr.to_socket_addrs() {
        Ok(addr_iter) => addr_iter,
        Err(_) => panic!("could not resolve socket address")
    }
        .next()
        .unwrap();

    let get_index = GetIndexHandler {};
    let get_contact = GetContactHandler {};
    let post_contact = PostContactHandler {};
    let not_found = NotFoundHandler {};


    // Create server
    let pool_size = 4;
    let handlers = std::collections::HashMap::new();
    let srvr = server::Server::build(socket_addr, pool_size, handlers)
        .expect("Unable to create Server")
        .add_handler(request::Request::GET(String::from("/")), get_index)
        .add_handler(request::Request::GET(String::from("contact")), get_contact)
        .add_handler(request::Request::GET(String::from("not_found")), not_found);
        // .add_handler(request::Request::POST(String::from("contact"), String::default()), post_contact);

    // run Server 
    srvr.run();

    Ok(())

}

