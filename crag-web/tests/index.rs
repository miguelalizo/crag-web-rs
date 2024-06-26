use anyhow::Result;
use crag_web::{handler, request, response, server::Server};

#[tokio::test]
async fn test_index() -> Result<()> {
    let server = Server::build()
        .register_error_handler(handler::default_error_404_handler)?
        .register_handler("/foo".into(), |_req: request::Request| {
            Ok(response::Response::Ok(
                "Bar!".into(),
                response::ContentType::HTML,
            ))
        })?
        .register_handler("/hello".into(), hello_handler)?
        .finalize(("127.0.0.1", 8010), 4)?;

    let _server_join = std::thread::spawn(move || {
        server.run().unwrap();
    });

    let r = reqwest::get("http://127.0.0.1:8010/bad").await?;
    assert!(r.status().is_client_error());

    let r = reqwest::get("http://127.0.0.1:8010/hello").await?;
    assert!(r.status().is_success());

    let r = reqwest::get("http://127.0.0.1:8010/foo").await?;
    assert!(r.status().is_success());

    assert_eq!(r.text().await?, "Bar!");

    Ok(())
}

// get "/hello"
fn hello_handler(_request: request::Request) -> anyhow::Result<response::Response> {
    Ok(response::Response::Ok(
        "Hello, Crag-Web!".into(),
        response::ContentType::HTML,
    ))
}
