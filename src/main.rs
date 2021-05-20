use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // should normally handle the errors

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) { // stream needs to be mutable because the stream's internal state might change (e.g. receiving more data)
    let mut buffer = [0; 1024]; // buffer management needs to be more complicated to handle requests of arbitrary size

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}