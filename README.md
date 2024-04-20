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
- **Multithreading with Built-in Threadpool**: Utilize the built-in threadpool, with custom Worker thread amounts, to handle concurrent requests efficiently.
- **Extensible**: Designed to be easily extendable with custom components.

## Installation

To use Crag-Web in your Rust project, add the following dependency to your `Cargo.toml`:

```toml
[dependencies]
crag-web = "0.1.0"
```

## Quick Start

```rust
use std::net::ToSocketAddrs;
use crag_web::{server, request, response};

// get "/hello"
fn hello_handler() -> response::Response {
    let body = "Hello, Crag-Web!";
    let status_line = "HTTP/1.1 200 OK";
    let len = body.len();

    // format http response
    let response = format!(
        "{status_line}\r\nContent-Length: {len}\r\n\r\n{body}"
    );
    response::Response{ content: response.as_bytes().to_vec() }
}

// get <bad request>
fn error_404_handler() -> response::Response {
    let body = "404 not found";
    let status_line = "HTTP/1.1 404 Not Found";
    let len = body.len();

    // format http response
    let response = format!(
        "{status_line}\r\nContent-Length: {len}\r\n\r\n{body}"
    );
    response::Response{ content: response.as_bytes().to_vec() }
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

    // Create server
    let pool_size = 4;
    let handlers = std::collections::HashMap::new();
    let app = server::Server::build(socket_addr, pool_size, handlers)
        .expect("Unable to create Server")
        .register_error_handler(error_404_handler)
        .register_handler(request::Request::GET(String::from("/hello")), hello_handler);

    // Run server
    app.run();

    Ok(())
}
```

This example creates a simple web server using Crag-Web that responds with "Hello, Crag-Web!" when accessing the `/hello` route.

## Documentation

For more details and advanced usage, check out the [Documentation](link/to/documentation).

## Contributing

We welcome contributions! Feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.


