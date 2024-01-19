use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

use hello_server::ThreadPool;

const HOST: &str = "127.0.0.1";
const PORT: &str = "8000";

fn main() {
    let url = format!("{HOST}:{PORT}");
    let listener = TcpListener::bind(url).unwrap();
    println!("listening on port {PORT}...");

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(&stream);
        });
    }
}

fn handle_connection(mut stream: &TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_status_line = buf_reader.lines().next().unwrap().unwrap();

    let (response_status_line, filename) = match request_status_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            std::thread::sleep(std::time::Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        response_status_line,
        contents.len(),
        contents
    );
    stream.write_all(response.as_bytes()).unwrap();
}
