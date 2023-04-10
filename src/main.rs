use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write}, fs, fmt::format, thread, time::Duration};

use webserv::ThreadPool;

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(t) => t,
        Err(_) => panic!("failed to bind.")
    };

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();


        pool.execute(|| {handle_connection(stream)});
        // println!("Connection established!");
    }
}


fn handle_connection(mut stream:TcpStream) {
    let buff_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    let request_line = buff_reader.lines().next().unwrap().unwrap();

    println!("Request line: {:#?}", request_line);
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}