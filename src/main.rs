use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write}, fs, time::Duration, thread};
use rust_web_server_hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Should be valid Listener");
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.expect("Should be valid stream");
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().expect("Should have next item").expect("Should be valid string");

    let (status_line, filename) = 
        match request_line.as_str() {
            "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
            "GET /sleep HTTP/1.1" => {
                thread::sleep(Duration::from_secs(5));
                ("HTTP/1.1 200 OK", "hello.html")
            }
            _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
        };

    let contents = fs::read_to_string(filename).expect("Should be able to read");
    let response = 
        format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, contents.len(), contents);

    stream.write_all(response.as_bytes()).expect("Should be able to write");
}

fn _print_request(buf_reader: BufReader<&mut TcpStream>) {
    let http_request: Vec<String> = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();

    println!("Request: {:#?}", http_request);
}
