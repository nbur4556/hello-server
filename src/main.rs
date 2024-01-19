use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

use hello_server::routing;
use hello_server::thread_pool::ThreadPool;

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
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let req_status_line = buf_reader.lines().next().unwrap().unwrap();

    let (res_status_line, filename) = routing::route_table(&req_status_line.as_str());
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        res_status_line,
        contents.len(),
        contents
    );
    stream.write_all(response.as_bytes()).unwrap();
}
