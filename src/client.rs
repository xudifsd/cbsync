extern crate clipboard;

use clipboard::ClipboardContext;

use std::time::Duration;
use std::thread;

fn main() {
    loop {
        let mut ctx = ClipboardContext::new().unwrap();
        println!("{}", ctx.get_contents().unwrap());
        thread::sleep(Duration::from_millis(1000));
    }
}
