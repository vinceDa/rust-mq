use std::net::TcpListener;
use std::println;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7086").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established");
    }


    // send_message
    fn send_message() {

    }
}