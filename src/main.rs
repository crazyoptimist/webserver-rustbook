use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    // println!("Request:\n{}", String::from_utf8_lossy(&buffer[..]))

    // Here's how to compose a HTTP response
    // HTTP-Version Status-Code Reason-Phrase CRLF
    // headers CRLF
    // message-body
    //
    // ex: HTTP/1.1 200 OK\r\n\r\n

    let method_get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(method_get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
