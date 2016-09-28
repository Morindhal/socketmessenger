extern crate ws;

use std::thread;

use ws::listen;
use ws::{connect, CloseCode};

fn main()
{
    thread::spawn(move || {
        listen
        ("127.0.0.1:3012",
            |out| {
            move |msg| {
                out.send(&*format!("{0}{1}{0}",msg," SVAR! "))
                }
            }
        ).unwrap(); 
    });

    std::thread::sleep(std::time::Duration::from_millis(1000));
    connect("ws://127.0.0.1:3012", |out| {
        out.send("Hello WebSocket").unwrap();

        move |msg| {
            println!("Got message: {}", msg);
            out.close(CloseCode::Normal)
        }
    }).unwrap()
}
