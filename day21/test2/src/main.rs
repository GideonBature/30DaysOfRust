use std::{sync::mpsc, thread};

enum ClientMessage { Incr, Get, Quit }

enum ServerMessage { Get(usize) }

fn main() {
    let (server_tx, client_rx) = mpsc::channel();
    let (client_tx, server_rx) = mpsc::channel();
    let server = thread::spawn(move || {
        let mut n = 0;
        loop {
            match server_rx.recv().unwrap() {
                ClientMessage::Quit => break,
                ClientMessage::Incr => n += 1,

                ClientMessage::Get => server_tx.send(ServerMessage::Get(n)).unwrap()
            }
        }
    });

    for msg in [ClientMessage::Incr, ClientMessage::Get, ClientMessage::Quit] {
        client_tx.send(msg).unwrap();
    }

    if let ServerMessage::Get(n) = client_rx.recv().unwrap() {
        println!("{}", n)
    }

    server.join().unwrap();
}
