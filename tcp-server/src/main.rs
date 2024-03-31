use std::{io::{BufRead, Write}, net::{TcpListener, TcpStream}};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8010")?;

    for stream in listener.incoming() {
        let stream = stream?;
        handle_connection(stream)?;
    }

    Ok(())

}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    println!("Connection established!");
    let buf = std::io::BufReader::new(&mut stream);

    let request_line = buf
        .lines()
        .next()
        .unwrap()?;
       
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "../static/html/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "../static/html/404.html")
    };

    let html_contents = std::fs::read_to_string(filename)?;
    let len = html_contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {len}\r\n\r\n{html_contents}"
    );

    stream
        .write_all(response.as_bytes())
        .unwrap();

    Ok(())

}
