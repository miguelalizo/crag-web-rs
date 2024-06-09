use crag_web::{handler::default_error_404_handler, request, response, server};

fn hello_handler(_request: request::Request) -> anyhow::Result<response::Response> {
    Ok(response::Response::Ok(
        "Hello, Crag-Web!".into(),
        response::ContentType::PLAIN,
    ))
}

fn main() -> anyhow::Result<()> {
    let app = server::Server::build()
        .register_error_handler(default_error_404_handler)?
        .register_handler("/hello".into(), hello_handler)?
        .register_handler("/foo".into(), |_req| {
            Ok(response::Response::Ok(
                "bar".into(),
                response::ContentType::PLAIN,
            ))
        })?
        .finalize(("127.0.0.1", 8010), 4)
        .unwrap();

    // Run server
    app.run()?;

    Ok(())
}
