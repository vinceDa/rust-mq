use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::ops::Deref;
use std::sync::Arc;
use serde_json::Value;

mod topic;
mod server;

use topic::message::Topic;
use crate::topic::message::{Broker, Message, MessageQueue};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7086").unwrap();

    let mut broker = Broker::new();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, &mut broker);
    }

    fn handle_connection(mut stream: TcpStream, broker: &mut Broker) {
        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();

        let request = String::from_utf8_lossy(&buffer[..]);
        println!("Request: {}", request);

        let url_send_message = b"POST /send_message HTTP/1.1\r\n";

        if buffer.starts_with(url_send_message) {
            // 找到 Content-Length 行获取 body 的长度
            let content_length = request.lines()
                .find(|line| line.starts_with("Content-Length:"))
                .and_then(|line| line.split_whitespace().nth(1))
                .and_then(|number| number.parse::<usize>().ok())
                .unwrap_or(0);
            // 找到 body 开始的位置，即第一个空行后
            let start_index = request.find("\r\n\r\n").unwrap() + 4;

            let body = &buffer[start_index..start_index + content_length];
            let body_str = String::from_utf8_lossy(body);
            println!("this is body: {}", body_str);

            let body_json: Value = serde_json::from_str(&body_str).unwrap();
            let topic_name = &body_json["topic"].to_string();
            let message = &body_json["message"].to_string();


            let topic = broker.get_topic_mut(topic_name).unwrap();

            let message_queue = topic.get_queue_mut("test_queue").unwrap();
            message_queue.push(Message::new(message.to_string()));

            if let Some(message) = message_queue.pop() {
                message.print_message();
            }


            let contents = fs::read_to_string("hello.html").unwrap();

            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                contents.len(),
                contents
            );
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}
