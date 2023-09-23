use std::net::TcpStream;
use std::io::{Read, Write};

fn main() {
    // Connect to the server on localhost:8080
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to server");

    // Send data to the server
    let message = "Hello from the client!";
    stream.write_all(message.as_bytes()).expect("Failed to write to server");

    // Read the server's response
    let mut response = String::new();
    stream.read_to_string(&mut response).expect("Failed to read response from server");

    println!("Received from server: {}", response);
}
