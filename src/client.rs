extern crate clipboard;
extern crate websocket;

extern crate rustc_serialize as serialize;

use std::str::from_utf8;
use websocket::client::request::Url;
use websocket::{Client, Message};
use websocket::Receiver as WebReceiver;
use websocket::Sender as WebSender;
use websocket::message::Type;
use serialize::json;

use clipboard::ClipboardContext;

use std::time::Duration;
use std::thread;

use std::io::prelude::*;
use std::fs::File;
use std::env;

use std::sync::mpsc::{channel, Sender, Receiver};

const CB_READER_SLEEP_MS: u64 = 1000;

fn read_server_ip() -> String {
    let default = "127.0.0.1".to_string();
    match env::home_dir() {
        None => default,
        Some(p) => {
            match File::open(p.join(".cbsyncrc")) {
                Err(_) => default,
                Ok(mut f) => {
                    let mut s = String::new();
                    match f.read_to_string(&mut s) {
                        Err(_) => default,
                        Ok(length) => s.trim().to_string()
                    }
                }
            }
        }
    }
}

fn web_adaptor(url: String, cb_receiver: Receiver<String>) {
    let ws_uri = Url::parse(&url[..]).unwrap();
    let request = Client::connect(ws_uri).unwrap();
    let response = request.send().unwrap();

    let mut ctx = ClipboardContext::new().unwrap();

    // TODO add retry
    match response.validate() {
        Ok(()) => (),
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };
    let (mut web_sender, mut web_receiver) = response.begin().split();

    thread::spawn(move || {
        loop {
            match cb_receiver.try_recv() {
                Ok(cb_content) => {
                    let response = Message::text(cb_content);
                    web_sender.send_message(&response).unwrap();
                }
                _ => ()
            }
            let message = web_receiver.recv_message(); // this will block, unless server send ping
            // TODO retry
            let message: Message = match message {
                Ok(message) => message,
                Err(e) => {
                    println!("Error: {:?}", e);
                    return;
                }
            };

            match message.opcode {
                Type::Text => {
                    ctx.set_contents(from_utf8(&*message.payload).unwrap().to_string());
                }
                Type::Binary => {
                    println!("received binary, which is not supported");
                }
                Type::Close => {
                    println!("server closed connection");
                    return
                }
                Type::Ping => {
                    println!("client received ping");
                    web_sender.send_message(&Message::ping(message.payload)).unwrap();
                }
                _ => (),
            }
        }
    });
}

fn main() {
    let ip = read_server_ip();
    let mut ctx = ClipboardContext::new().unwrap();
    let url = format!("{}{}{}", "ws://", ip, ":31415");

    let (sender, receiver) = channel();

    web_adaptor(url, receiver);

    let mut ctx = ClipboardContext::new().unwrap();
    let mut content = "".to_string();
    loop {
        let mut cur = ctx.get_contents().unwrap();

        if content != cur {
            content = cur;
            sender.send(content.clone());
        }

        thread::sleep(Duration::from_millis(CB_READER_SLEEP_MS));
    }
}
