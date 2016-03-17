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

fn main() {
    let mut ctx = ClipboardContext::new().unwrap();
    let url = "ws://127.0.0.1:9001".to_string();
    let agent = "rust-websocket";

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
