use std::net::ToSocketAddrs;

mod threadpool;
mod server;
mod request;
mod handler;
mod response;


fn not_found() -> response::Response {
    let filename = "../static/html/404.html";
    let html = std::fs::read_to_string(filename).unwrap();//?;;
    // let html_contents = std::fs::read_to_string(filename).unwrap();//?;
    let status_line = "HTTP/1.1 404 NOT FOUND";
    let len = html.len();

    // format http response
    let response = format!(
        "{status_line}\r\nContent-Length: {len}\r\n\r\n{html}"
    );

    response::Response { content: response }
}

fn index() -> response::Response {
    let filename = "../static/html/index.html";
    let html = std::fs::read_to_string(filename).unwrap();//?;;
    // let html_contents = std::fs::read_to_string(filename).unwrap();//?;
    let status_line = "HTTP/1.1 200 OK";
    let len = html.len();

    // format http response
    let response = format!(
        "{status_line}\r\nContent-Length: {len}\r\n\r\n{html}"
    );

    response::Response { content: response }}

fn css_default() -> response::Response {
    let filename = "../static/css/default.css";
    let html = std::fs::read_to_string(filename).unwrap();//?;;
    // let html_contents = std::fs::read_to_string(filename).unwrap();//?;
    let status_line = "HTTP/1.1 200 OK";
    let len = html.len();

    // format http response
    let response = format!(
        "{status_line}\r\nContent-Length: {len}\r\n\r\n{html}"
    );

    response::Response { content: response }
}

fn css_blue() -> response::Response {
    let filename = "../static/css/blue.css";
    let html = std::fs::read_to_string(filename).unwrap();//?;;
    // let html_contents = std::fs::read_to_string(filename).unwrap();//?;
    let status_line = "HTTP/1.1 200 OK";
    let len = html.len();

    // format http response
    let response = format!(
        "{status_line}\r\nContent-Length: {len}\r\n\r\n{html}"
    );

    response::Response { content: response }
}

fn css_green() -> response::Response {
    let filename = "../static/css/green.css";
    let html = std::fs::read_to_string(filename).unwrap();//?;;
    // let html_contents = std::fs::read_to_string(filename).unwrap();//?;
    let status_line = "HTTP/1.1 200 OK";
    let len = html.len();

    // format http response
    let response = format!(
        "{status_line}\r\nContent-Length: {len}\r\n\r\n{html}"
    );

    response::Response { content: response }
}

fn css_purple() -> response::Response {
    let filename = "../static/css/purple.css";
    let html = std::fs::read_to_string(filename).unwrap();//?;;
    // let html_contents = std::fs::read_to_string(filename).unwrap();//?;
    let status_line = "HTTP/1.1 200 OK";
    let len = html.len();

    // format http response
    let response = format!(
        "{status_line}\r\nContent-Length: {len}\r\n\r\n{html}"
    );

    response::Response { content: response }
}

fn js_script() -> response::Response {
    let filename = "../static/scripts/script.js";
    let html = std::fs::read_to_string(filename).unwrap();//?;;
    // let html_contents = std::fs::read_to_string(filename).unwrap();//?;
    let status_line = "HTTP/1.1 200 OK";
    let len = html.len();

    // format http response
    let response = format!(
        "{status_line}\r\nContent-Length: {len}\r\n\r\n{html}"
    );

    response::Response { content: response }
}

fn image_me() -> response::Response {
    let filename = "../static/images/me.jpeg";
    let html = std::fs::read_to_string(filename).unwrap();//?;;
    // let html_contents = std::fs::read_to_string(filename).unwrap();//?;
    let status_line = "HTTP/1.1 200 OK";
    let len = html.len();

    // format http response
    let response = format!(
        "{status_line}\r\nContent-Length: {len}\r\n\r\n{html}"
    );

    response::Response { content: response }
}

fn image_linkedin() -> response::Response {
    let filename = "../static/images/linkedin.jpeg";
    let html = std::fs::read_to_string(filename).unwrap();//?;;
    // let html_contents = std::fs::read_to_string(filename).unwrap();//?;
    let status_line = "HTTP/1.1 200 OK";
    let len = html.len();

    // format http response
    let response = format!(
        "{status_line}\r\nContent-Length: {len}\r\n\r\n{html}"
    );

    response::Response { content: response }
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

    let srvr = server::Server::build(socket_addr, pool_size, handlers)
        .expect("Unable to create Server")
        .add_handler(request::Request::GET(String::from("/")), index)
        .add_handler(request::Request::GET(String::from("/not_found")), not_found)
        .add_handler(request::Request::GET(String::from("/scripts/script.js")), js_script)
        .add_handler(request::Request::GET(String::from("/css/default.css")), css_default)
        .add_handler(request::Request::GET(String::from("/css/blue.css")), css_blue)
        .add_handler(request::Request::GET(String::from("/css/green.css")), css_green)
        .add_handler(request::Request::GET(String::from("/css/purple.css")), css_purple);
        // .add_handler(request::Request::GET(String::from("/images/me.jpeg")), image_me)
        // .add_handler(request::Request::GET(String::from("/images/linkedin.jpeg")), image_linkedin);
        // .add_handler(request::Request::POST(String::from("contact"), String::default()), post_contact);

    // run Server 
    srvr.run();

    Ok(())

}

