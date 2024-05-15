use anyhow::Result;
use crag_web::{handler, request, response, server::Server};

#[tokio::test]
async fn test_index() -> Result<()> {
    let server = Server::build()
        .register_error_handler(handler::default_error_404_handler)
        .register_handler(request::Request::GET(String::from("/hello")), hello_handler)
        .finalize(("127.0.0.1", 8010), 4)?;

    let _server_join = std::thread::spawn(move || {
        server.run();
    });

    let r = reqwest::get("http://127.0.0.1:8010/bad").await?;
    assert!(r.status().is_client_error());

    let r = reqwest::get("http://127.0.0.1:8010/hello").await?;
    assert!(r.status().is_success());

    assert_eq!(r.text().await?, "Hello, Crag-Web!");

    Ok(())
}

// get "/hello"
fn hello_handler(_request: request::Request) -> response::Response {
    response::Response::Ok("Hello, Crag-Web!".to_owned())
}
