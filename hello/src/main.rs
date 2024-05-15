use crag_web::{request, response, server};

// get "/hello"
fn hello_handler(_request: request::Request) -> response::Response {
    let body = "Hello, Crag-Web!";
    let status_line = "HTTP/1.1 200 OK";
    let len = body.len();

    // format http response
    let response = format!("{status_line}\r\nContent-Length: {len}\r\n\r\n{body}");
    response::Response {
        content: response.as_bytes().to_vec(),
    }
}

// get <bad request>
fn error_404_handler(_request: request::Request) -> response::Response {
    let body = "404 not found";
    let status_line = "HTTP/1.1 404 Not Found";
    let len = body.len();

    // format http response
    let response = format!("{status_line}\r\nContent-Length: {len}\r\n\r\n{body}");
    response::Response {
        content: response.as_bytes().to_vec(),
    }
}

fn main() -> std::io::Result<()> {
    let app = server::Server::build()
        .register_error_handler(error_404_handler)
        .register_handler(request::Request::GET(String::from("/hello")), hello_handler)
        .finalize(("127.0.0.1", 8010), 4)
        .unwrap();

    // Run server
    app.run();

    Ok(())
}
