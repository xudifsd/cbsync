extern crate clipboard;
extern crate websocket;

extern crate rustc_serialize as serialize;

use std::str::from_utf8;
use websocket::client::request::Url;
use websocket::{Client, Message, Sender, Receiver};
use websocket::message::Type;
use serialize::json;

use clipboard::ClipboardContext;

use std::time::Duration;
use std::thread;

use std::io::prelude::*;
use std::fs::File;
use std::env;

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

fn main() {
    let ip = read_server_ip();
    let mut ctx = ClipboardContext::new().unwrap();
    let url = format!("{}{}{}", "ws://", ip, ":31415");

    let ws_uri = Url::parse(&url[..]).unwrap();
    let request = Client::connect(ws_uri).unwrap();
    let response = request.send().unwrap();

    match response.validate() {
        Ok(()) => (),
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    }

    let (mut sender, mut receiver) = response.begin().split();

    loop {
        sender.send_message(&Message::text(ctx.get_contents().unwrap()));

        thread::sleep(Duration::from_millis(1000));
    }
}
