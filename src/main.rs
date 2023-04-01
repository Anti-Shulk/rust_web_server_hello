use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write}, fs};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Should be valid Listener");
    for stream in listener.incoming() {
        let stream = stream.expect("Should be valid stream");
        println!("Connection established!");

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().expect("Should have next item").expect("Should be valid string");

    match request_line.as_str() {
        "GET / HTTP/1.1" => {
            let status_line = "HTTP/1.1 200 OK";
            let contents = fs::read_to_string("hello.html").expect("Should be able to read");
            let response = 
                format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, contents.len(), contents);

            stream.write_all(response.as_bytes()).expect("Should be able to write");
        },
        _ => {
            let status_line = "HTTP/1.1 404 NOT FOUND";
            let contents = fs::read_to_string("404.html").expect("Should be able to read");
            let response = 
                format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, contents.len(), contents);

            stream.write_all(response.as_bytes()).expect("Should be able to write");
        },
    }
}

fn _print_request(buf_reader: BufReader<&mut TcpStream>) {
    let http_request: Vec<String> = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();

    println!("Request: {:#?}", http_request);
}

