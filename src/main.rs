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

    if buffer.starts_with(method_get) {
        let contents = fs::read_to_string("index.html").unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let contents = fs::read_to_string("404.html").unwrap();

        let response = format!(
            "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
