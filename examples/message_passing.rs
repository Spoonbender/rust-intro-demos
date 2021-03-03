use std::sync::mpsc;
use std::thread;
use std::{
    io::{self, BufRead},
    sync::mpsc::{Receiver, Sender},
};

struct ChatMessage {
    text: String,
    index: u64,
}

fn main() {
    let (tx, rx): (Sender<ChatMessage>, Receiver<ChatMessage>) = mpsc::channel();

    let ingress_thread = thread::spawn(move || {
        let mut counter = 0;
        for line in io::stdin().lock().lines() {
            let msg = ChatMessage {
                text: line.unwrap(),
                index: counter,
            };
            tx.send(msg).unwrap();
            counter += 1;
        }
    });

    let processor_thread = thread::spawn(move || loop {
        let msg = rx.recv().unwrap();
        println!("Msg #{}: <BEGIN>{}<END>", msg.index, msg.text.trim());
    });

    println!("Start posting messages!");
    processor_thread.join().unwrap();
    ingress_thread.join().unwrap();
}
