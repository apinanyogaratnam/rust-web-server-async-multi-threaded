use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
extern crate futures;
use futures::executor::ThreadPoolBuilder;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap(); // bind listener

    let mut pool_builder = ThreadPoolBuilder::new();
    pool_builder.pool_size(100);
    let pool = pool_builder.create().expect("couldn't create thread pool");

    // Listen for an incoming connection.
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // spawning each connection in a new thread asynchronously
        pool.spawn_ok(async {
            handle_connection(stream).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream) {
    // Read the first 1024 bytes of data from the stream
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string("hello.html").unwrap();

    let response = format!("HTTP/1.1 200 OK\r\nConnection: keep-alive\r\nContent-Length: {}\r\nContent-Type: text/html; charset=utf-8\r\n\r\n{}", contents.len(), contents);

    stream.write(response.as_bytes()).unwrap(); // write response
    stream.flush().unwrap();
}
