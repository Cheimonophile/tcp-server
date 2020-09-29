use std::net;
use std::io;
use std::io::prelude::*;

pub fn start() -> io::Result<()> {

    // creates the tcp listener
    let listener = net::TcpListener::bind("0.0.0.0:12345")?;

    // prints that the server has started
    println!("Server Started!\n");

    // accept connections and process them serially
    for stream in listener.incoming() {
        let mut stream = stream?;

        // buffer to hold the message
        let mut message= String::new();

        // reads the message into the message variable
        stream.read_to_string(&mut message)?;

        // prints the message
        println!("{}", message);
    }
    return Ok(());
}