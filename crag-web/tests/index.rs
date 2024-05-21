use anyhow::Result;
use crag_web::{handler, request, response, server::Server};

#[tokio::test]
async fn test_index() -> Result<()> {
    let server = Server::build()
        .register_error_handler(Box::new(handler::default_error_404_handler))
        .register_handler(
            request::Request::GET("/foo".to_owned()),
            Box::new(|_req| Ok(response::Response::Ok("Bar!".to_owned()))),
        )
        .register_handler(
            request::Request::GET(String::from("/hello")),
            Box::new(hello_handler),
        )
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
    Ok(response::Response::Ok("Hello, Crag-Web!".to_owned()))
}
