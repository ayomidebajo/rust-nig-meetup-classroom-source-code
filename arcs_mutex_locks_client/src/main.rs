use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    // three different messages from three clients
    let messages_batch = ["first message from client at 0", "second message from client at 1", "third message from client at 2"];

            for i in 0..3 {
                // create a new stream connection for every iteration
                let stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to server");
                // parse stream connection and a different message for every iteration
                client_handler(messages_batch[i], stream);
            }
}

fn client_handler(message: &str, mut stream: TcpStream) {

    // write to stream with a message (each messages are different)
    stream.write_all(message.as_bytes()).expect("Failed to write to server");

    // Read the server's response, this logs the shared data which is the overall data/state of the application
    let mut response = String::new();
    
    stream
        .read_to_string(&mut response)
        .expect("Failed to read response from server");

    println!("Received from server: {}", response);
}
