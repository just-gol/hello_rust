use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
fn main() {
    let listener = TcpListener::bind("0.0.0.0:8888")
        .expect("无法绑定到 0.0.0.0:7878,请确认地址和端口是否可用");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("connection failed :{}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received:\n{}", request);
    let (status_line, filename) = if request.starts_with("GET / ") {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let contents =
        fs::read_to_string(filename).unwrap_or_else(|_| "Error reading file".to_string());
    let response = format!(
        "{status_line}\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
}
