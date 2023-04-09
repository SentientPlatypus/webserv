use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write}, fs, fmt::format};

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(t) => t,
        Err(_) => panic!("failed to bind.")
    };

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
        // println!("Connection established!");
    }
}


fn handle_connection(mut stream:TcpStream) {
    let buff_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    let request_line = buff_reader.lines().next().unwrap().unwrap();

    println!("Request line: {:#?}", request_line);
    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/2.2 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();
    
        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );
    
        stream.write_all(response.as_bytes()).unwrap();    
    } 
    else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let content = "L NO PAGE";
        let length = content.len();
        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{content}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    }
}