# crag-web

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/crag-web.svg
[crates-url]: https://crates.io/crates/crag-web
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/miguelalizo/crag-web-rs/blob/main/LICENSE


Crag-Web is a lightweight and flexible HTTP web server framework written in Rust. Inspired by the diverse routes found at climbing crags, Crag-Web allows you to define and handle various HTTP routes with ease, making it simple to build powerful web applications.

## Features

- **Simple Routing**: Define routes for handling HTTP requests with ease.
- **Multithreading with Built-in Threadpool**: Defines a built-in threadpool, with custom Worker thread amounts, to handle concurrent requests efficiently.
- **Extensible**: Designed to be easily extendable with custom components.

## Quick Start

```rust
use crag_web::{request, response, server};

// get "/hello"
fn hello_handler(_request: request::Request) -> anyhow::Result<response::Response> {
    Ok(response::Response::Ok(
        "Hello, Crag-Web!".into(),
        response::ContentType::HTML,
    ))
}

// get <bad request>
fn error_404_handler(_request: request::Request) -> anyhow::Result<response::Response> {
    Ok(response::Response::NotFound(("404 Not Found").into()))
}

fn main() -> anyhow::Result<()> {
    let app = server::Server::build()
        .register_error_handler(Box::new(error_404_handler))?
        .register_handler(
            request::Request::GET(String::from("/hello")),
            Box::new(hello_handler),
        )?
        .finalize(("127.0.0.1", 8010), 1)
        .unwrap();

    // Run server
    app.run()?;

    Ok(())
}

```

This example creates a simple web server using Crag-Web that responds with "Hello, Crag-Web!" when accessing the `/hello` route.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.


