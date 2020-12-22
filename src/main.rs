use std::{
    io::prelude::*,
    thread,
    fs,
    net::{TcpListener, TcpStream},
    time::Duration,
};

use https_verifier::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1337").unwrap();

    match ThreadPool::new(4) {
        Ok(pool) => {
            for stream in listener.incoming() {
                let stream = stream.unwrap();

                pool.execute(|| {
                    handle_connection(stream);
                });
            }           
        },
        Err(pool_error) => {
            println!("Failed to create pool: ");
        }
    };
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let (status_line, filename) = if buffer.starts_with(b"GET / HTTP/1.1") {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(b"GET /sleep HTTP/1.1") {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
