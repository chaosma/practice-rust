//! A simple echo tcp server
//! (one terminal) server side: cargo run
//! (another terminal) client side: telnet 127.0.0.1 8080

use std::io::{Read, Result, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

// handle incoming tcp connection
fn handle_connection(stream: &mut TcpStream) -> Result<()> {
    // define buffer with maximum 256 bytes
    let mut buf = [0u8; 256];
    loop {
        match stream.read(&mut buf) {
            // read from incoming socket
            Ok(n) => {
                let _ = stream.write(&buf[0..n])?; // echo back whatever we have read
            }
            Err(e) => {
                // handle socket read error
                println!("cannot read the incoming connection, error = {}", e);
            }
        }
    }
}

fn main() {
    // bind listener to local address at port 8080
    let listener = match TcpListener::bind("127.0.0.1:8080") {
        Ok(l) => l,
        Err(e) => {
            println!("unable to bind address 127.0.0.1:8080, error = {}", e);
            return;
        }
    };
    println!("listening on: 127.0.0.1:8080");

    // listening on incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                // spawn a new thread for each connection, so that we can handle multiple clients
                thread::spawn(move || {
                    println!("New connection from: {}", s.peer_addr().unwrap());
                    match handle_connection(&mut s) {
                        Ok(_) => {}
                        // handle socket write error to avoid compiler warning
                        Err(e) => println!("write to socket error: {}", e),
                    }
                });
            }
            Err(e) => {
                println!("couldn't handle the incoming stream, error = {}", e);
            }
        }
    }
}
