extern crate websocket;

use std::thread;
use std::str::from_utf8;
use websocket::{Server, Message, Sender, Receiver};
use websocket::message::Type;

use std::time::Duration;

const SERVER_SLEEP_MS: u64 = 1000;

fn main() {
    let addr = "0.0.0.0:31415".to_string();

    let server = Server::bind(&addr[..]).unwrap();

    for connection in server {
        thread::spawn(move || {
            let request = connection.unwrap().read_request().unwrap();
            request.validate().unwrap();
            let response = request.accept();
            let (mut sender, mut receiver) = response.send().unwrap().split();

            let ping = Message::ping(vec![]);
            sender.send_message(&ping).unwrap();

            for message in receiver.incoming_messages() {
                let message: Message = match message {
                    Ok(message) => message,
                    Err(e) => {
                        println!("{:?}", e);
                        let _ = sender.send_message(&Message::close());
                        return;
                    }
                };

                match message.opcode {
                    Type::Text => {
                        let response = Message::text(from_utf8(&*message.payload).unwrap());
                        println!("{}", from_utf8(&*message.payload).unwrap());
                        sender.send_message(&response).unwrap()
                    },
                    Type::Binary => println!("received binary, which is not supported"),
                    Type::Close => {
                        let _ = sender.send_message(&Message::close());
                        return;
                    }
                    Type::Ping => {
                        println!("server received ping");
                        let message = Message::ping(message.payload);
                        sender.send_message(&message).unwrap();
                        thread::sleep(Duration::from_millis(SERVER_SLEEP_MS))
                    }
                    _ => (),
                }
            }
        });
    }
}
