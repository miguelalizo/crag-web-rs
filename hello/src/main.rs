use crag_web::{request, response, server};

// get "/hello"
fn hello_handler(_request: request::Request) -> response::Response {
    response::Response::Ok("Hello, Crag-Web!".to_owned())
}

// get <bad request>
fn error_404_handler(_request: request::Request) -> response::Response {
    response::Response::NotFound(("404 Not Found").to_owned())
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
