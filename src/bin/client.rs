use std::{
    io::{stdin, Write},
    net::TcpStream,
};

#[allow(unused_variables)]
fn main() {
    let address = "localhost:8080";

    let mut incomming =
        TcpStream::connect(address).expect("\u{1b}[34mFailed to connect to server\u{001b}[0m");
    let mut buffer = String::new();

    loop {
        println!("\u{1b}[34m->");

        let bytes_written = stdin()
            .read_line(&mut buffer)
            .expect("\u{1b}[34mFailed to read message from stdin(from client)\u{001b}[0m");

        println!("-\u{001b}[0m");

        println!("test");
        println!("sending message: {buffer}");
        incomming
            .write(buffer.as_bytes())
            .expect("\u{1b}[34mFailed to send message to the server\u{001b}[0m");

        // println!("message sent");
        // println!("current buffer: {}", buffer);

        if buffer.trim() == ":quit" {
            // println!("Shut down signal recieved");

            // closing the connection from the client side
            incomming
                .shutdown(std::net::Shutdown::Both)
                .expect("Failed to shutdown the connection");
            break;
        }
        // println!("clearing the buffer");
        buffer.clear();
    }
}
