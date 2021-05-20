use std::net::{TcpListener, TcpStream};
use std::{fs, io::prelude::*};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // should normally handle the errors

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // stream needs to be mutable because the stream's internal state might change (e.g. receiving more data)
    let mut buffer = [0; 1024]; // buffer management needs to be more complicated to handle requests of arbitrary size

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let contents = fs::read_to_string("hello.html").unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", // Content-Length header added to make the response valid HTTP.
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}", // Content-Length header added to make the response valid HTTP.
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

// fn handle_connection(mut stream: TcpStream) { // stream needs to be mutable because the stream's internal state might change (e.g. receiving more data)
//     let mut buffer = [0; 1024]; // buffer management needs to be more complicated to handle requests of arbitrary size

//     stream.read(&mut buffer).unwrap();

//     println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

//     let contents = fs::read_to_string("hello.html").unwrap();

//     let response = format!(
//         "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", // Content-Length header added to make the response valid HTTP.
//         contents.len(),
//         contents
//     );
//     stream.write(response.as_bytes()).unwrap();
//     stream.flush().unwrap();
// }
