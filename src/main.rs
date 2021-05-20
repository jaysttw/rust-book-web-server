use std::net::{TcpListener, TcpStream};
use std::{fs, io::prelude::*};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // should normally handle the errors
    let pool = ThreadPool::new();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");

        pool.execute(|| {
            // skipping over thread spawn example in Listing 20-11.
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    // stream needs to be mutable because the stream's internal state might change (e.g. receiving more data)
    let mut buffer = [0; 1024]; // buffer management needs to be more complicated to handle requests of arbitrary size

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}", // Content-Length header added to make the response valid HTTP.
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
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
