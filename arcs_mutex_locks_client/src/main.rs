use std::io::{Read, Write};
use std::net::TcpStream;
// use std::thread::spawn;

fn main() {
    // Connect to the server on localhost:8080
    // let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to server");

    // Send data to the server
    // let message = "Hello from the client another one!";
    // stream.write_all(message.as_bytes()).expect("Failed to write to server");

    // three different messages from three clients
    let messages_batch = ["first message from client at 0", "second message from client at 1", "third message from client at 2"];

    // loop logic
   
        // spawn(move || {
            for i in 0..3 {
                let stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to server");
                client_handler(messages_batch[i], stream);
            }
        // });

    // // Read the server's response
    // let mut response = String::new();
    // stream.read_to_string(&mut response).expect("Failed to read response from server");

    // println!("Received from server: {}", response);
}

fn client_handler(message: &str, mut stream: TcpStream) {

    // Send data to the server
    // let message = "Hello from the client another one!";
    stream.write_all(message.as_bytes()).expect("Failed to write to server");

    // Read the server's response
    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .expect("Failed to read response from server");

    println!("Received from server: {}", response);
}
