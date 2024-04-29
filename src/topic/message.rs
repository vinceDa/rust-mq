use std::collections::HashMap;

pub struct Message {
    content: String,
}

impl Message {
    pub fn new(content: String) -> Message {
        Message {
            content,
        }
    }

    pub fn print_message(&self) {
        println!("message: {}", self.content);
    }
}

pub struct MessageQueue {
    messages: Vec<Message>,
    max: usize,
    size: usize,
    consumer_index: usize,
    produce_index: usize,
}

impl MessageQueue {
    pub fn new() -> MessageQueue {
        MessageQueue {
            messages: Vec::with_capacity(128),
            max: 128,
            size: 0,
            consumer_index: 0,
            produce_index: 0,
        }
    }
    pub fn push(&mut self, message: Message) {
        if (self.size == self.max) {
            println!("message queue is full");
            return;
        }

        self.messages.push(message);
        self.size += 1;
        self.produce_index = (self.produce_index + 1) % self.max;
    }

    pub fn pop(&mut self) -> Option<&Message> {
        if (self.size == 0) {
            return None;
        }

        let message = self.messages.get(self.consumer_index);
        self.consumer_index = (self.consumer_index + 1) % self.max;

        self.size -= 1;

        message
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

#[cfg(test)]
mod message_tests {
    use crate::topic::message::{Message, MessageQueue};

    #[test]
    fn push_pop_message() {
        let mut message_queue = MessageQueue::new();

        for i in 0..128 {
            let message = Message::new(format!("Hello MQ! -- {}", i));
            message_queue.push(message);
        }

        while !message_queue.is_empty() {
            match message_queue.pop() {
                Some(message) => {
                    println!("{}", message.content)
                }
                None => {}
            };
        }
    }
}

pub struct Topic {
    name: String,
    queues_map: HashMap<String, MessageQueue>,
}

impl Topic {
    pub fn new(name: &str) -> Self {
        Topic {
            name: name.to_string(),
            queues_map: HashMap::new(),
        }
    }

    fn add_queue(&mut self, queue_name: &str) {
        self.queues_map.insert(queue_name.to_string(), MessageQueue::new());
    }

    fn remove_queue(&mut self, queue_name: &str) {
        self.queues_map.remove(queue_name);
    }

    pub fn get_queue(&mut self, queue_name: &str) -> Option<&MessageQueue> {
        match self.queues_map.get(queue_name) {
            None => {
                self.add_queue(queue_name);
                self.queues_map.get(queue_name)
            }
            _ => {
                self.queues_map.get(queue_name)
            }
        }
    }

    pub fn get_queue_mut(&mut self, queue_name: &str) -> Option<&mut MessageQueue> {
        match self.queues_map.get(queue_name) {
            None => {
                self.add_queue(queue_name);
                self.queues_map.get_mut(queue_name)
            }
            _ => {
                self.queues_map.get_mut(queue_name)
            }
        }
    }
}


pub struct Broker {
    topics: HashMap<String, Topic>,
}

impl Broker {
    pub fn new() -> Self {
        Broker {
            topics: HashMap::new()
        }
    }

    pub fn add_topic(&mut self, topic_name: &str, topic: Topic) {
        self.topics.insert(topic_name.to_string(), topic);
    }

    fn add_new_topic(&mut self, topic_name: &str) {
        self.topics.insert(topic_name.to_string(), Topic::new(topic_name));
    }

    pub fn get_topic(&mut self, topic_name: &str) -> Option<&Topic> {
        match self.topics.get(topic_name) {
            None => {
                self.add_new_topic(topic_name);
                self.topics.get(topic_name)
            }
            Some(topic) => {
                self.topics.get(topic_name)
            }
        }
    }

    pub fn get_topic_mut(&mut self, topic_name: &str) -> Option<&mut Topic> {
        match self.topics.get(topic_name) {
            None => {
                self.add_new_topic(topic_name);
                self.topics.get_mut(topic_name)
            }
            Some(topic) => {
                self.topics.get_mut(topic_name)
            }
        }
    }
}
