use std::io::prelude::*;
use std::net::{TcpStream, TcpListener};
use std::fs::File;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7555")
        .unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream
        .read(&mut buffer)
        .unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) =
        match buffer.starts_with(get) {
            true => ("HTTP/1.1 200 OK\r\n\r\n", "hello.html"),
            _ => ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html"),
        };


    let mut file = File::open(filename)
        .unwrap();

    let mut contents = String::new();

    file
        .read_to_string(&mut contents)
        .unwrap();

    let response = format!("{}{}", status_line, contents);

    stream
        .write(response.as_bytes())
        .unwrap();

    stream
        .flush()
        .unwrap();
}