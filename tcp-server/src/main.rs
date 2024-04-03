use std::{io::{BufRead, Write}, net::{TcpListener, TcpStream}};
use tcp_server::ThreadPool;

fn main() -> std::io::Result<()> {
    // TcpListener bound to a port
    let listener = TcpListener::bind("127.0.0.1:8010")?;
    // ThreadPool instantiated with finite capacity 
    let pool = ThreadPool::build(4)
        .expect("ThreadPool size must be greater than 0");
    

    for stream in listener.incoming() {
        let stream = stream?;

        pool.execute( || {
            handle_connection(stream); //?
        });
    }

    Ok(())

}

fn handle_connection(mut stream: TcpStream){ //} -> std::io::Result<()> { 
    // create buffer to store stream lines   
    let buf = std::io::BufReader::new(&mut stream);

    let request_line = buf
        .lines()
        .next()
        .unwrap().unwrap();//?;
       
    // serve a response based on the request line
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "../static/html/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "../static/html/404.html")
    };

    // read html file contents into a String
    // and get len
    let html_contents = std::fs::read_to_string(filename).unwrap();//?;
    let len = html_contents.len();

    // format http response
    let response = format!(
        "{status_line}\r\nContent-Length: {len}\r\n\r\n{html_contents}"
    );

    // write response into TcpStream
    stream
        .write_all(response.as_bytes()).unwrap();//?;

    
    // Ok(())

}
