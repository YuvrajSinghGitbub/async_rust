use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    // setting up the tcp stream
    let address = "localhost:8080";
    let tcp_stream = TcpListener::bind(address).unwrap();

    println!("Listening for messages on \u{1b}[32m{}\u{001b}[0m", address);

    // send each stream to a seprate thread
    for stream in tcp_stream.incoming() {
        let stream = stream.unwrap();
        println!("New connection recieved...");

        thread::spawn(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    // TODO: for reach each msg sent by the client and just echo it back
    let mut reader = BufReader::new(&stream);

    loop {
        // println!("Reading from stream");
        let thing = reader.fill_buf().unwrap().to_vec();
        let bytes_read = thing.len();
        reader.consume(bytes_read);
        // println!("Stream read");

        if bytes_read == 0 {
            break;
        }

        // do some processing
        let msg = String::from_utf8(thing).unwrap();
        println!("\u{1b}[33m-> {}\u{001b}[0m", msg.trim());

        if msg.trim() == ":quit" {
            println!("Breaking the connection");

            // send a termination signal to the client
            stream
                .write(b":quit")
                .expect("\u{1b}[34mFailed to send exit message to the client\u{001b}[0m");
            break;
        }
    }

    println!("connection terminated");
    stream.flush().unwrap();
}
