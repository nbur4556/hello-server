use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_status_line = buf_reader.lines().next().unwrap().unwrap();

    let mut response_status_line = "HTTP/1.1 200 OK";
    let contents = if request_status_line == "GET / HTTP/1.1" {
        fs::read_to_string("index.html").unwrap()
    } else {
        response_status_line = "HTTP/1.1 404 Not Found";
        fs::read_to_string("404.html").unwrap()
    };

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        response_status_line,
        contents.len(),
        contents
    );
    stream.write_all(response.as_bytes()).unwrap();
}
