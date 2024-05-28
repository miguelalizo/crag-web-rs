use crag_web::{request, response, server};

// get "/hello"
fn hello_handler(_request: request::Request) -> anyhow::Result<response::Response> {
    Ok(response::Response::Ok(
        "Hello, Crag-Web!".to_owned().into(),
        response::ContentType::HTML,
    ))
}

// get <bad request>
fn error_404_handler(_request: request::Request) -> anyhow::Result<response::Response> {
    Ok(response::Response::NotFound(
        ("404 Not Found").to_owned().into_bytes(),
    ))
}

fn main() -> anyhow::Result<()> {
    let app = server::Server::build()
        .register_error_handler(Box::new(error_404_handler))?
        .register_handler(
            request::Request::GET(String::from("/hello")),
            Box::new(hello_handler),
        )?
        .finalize(("127.0.0.1", 8010), 4)
        .unwrap();

    // Run server
    app.run()?;

    Ok(())
}
