struct Message {
    content: String,
}

impl Message {
    pub fn new(content: String) -> Message {
        Message {
            content,
        }
    }
}

struct MessageQueue {
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

    pub fn pop(&mut self) -> Option<Message> {
        if (self.size == 0) {
            return None;
        }

        let message = Some(self.messages.remove(self.consumer_index));
        self.consumer_index = (self.consumer_index + 1) % self.max;

        self.size -= 1;

        message
    }
}

#[cfg(test)]
mod message_tests {

    use super::*;

    #[test]
    fn push_message() {
        Message::new("Hello MQ!".to_string())
    }
}

struct Topic {
    message_queues: Vec<MessageQueue>,
}

