use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        handle_connection(&mut stream);
    }
}

fn handle_connection(stream: &mut TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("{}", String::from_utf8_lossy(&buffer));
    println!("request From: {}", stream.peer_addr().unwrap().to_string());

    let contents = fs::read_to_string("index.html").unwrap();

    let response = format!("HTTP/1.1 200 OK\r\nContent-length: {}\r\n\r\n{}", contents.len(), contents);

    stream.write( response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
