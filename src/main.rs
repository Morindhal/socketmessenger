extern crate ws;

use std::thread;

use ws::listen;
use ws::{connect, CloseCode};

fn main()
{
/*
*This is the server-part of the chat-program, currently simply echoes back what was recieved.
*It is here that chat messages should be presented to the user.
**/
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

/*
*This is the client-part of the chat-program, currently simply sends one message and then ends the program.
*It is here that input should be placed into the program.
*Should I have this in a thread seperate from the main thread as well or simply have a main loop in the main-thread?
**/
    std::thread::sleep(std::time::Duration::from_millis(1000));
    connect("ws://127.0.0.1:3012", |out| {
        out.send("Hello WebSocket").unwrap();

        move |msg| {
            println!("Got message: {}", msg);
            out.close(CloseCode::Normal)
        }
    }).unwrap()
}
