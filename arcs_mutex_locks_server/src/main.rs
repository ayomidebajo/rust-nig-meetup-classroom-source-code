use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::io::{Read, Write};
use std::thread;

fn main() {
    // Create a mutable data structure wrapped in an Arc and Mutex for shared ownership and synchronization.
    let shared_data = Arc::new(Mutex::new(0));

    // Create a TcpListener that listens on localhost:8080
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");

    println!("Server listening on port 8080...");

    // Accept incoming connections and handle them in separate threads
    for stream in listener.incoming() {
        let shared_data_clone = Arc::clone(&shared_data);

        match stream {
            Ok(stream) => {
                println!("Accepted a connection.");

                // Spawn a new thread to handle the client
                thread::spawn(move || {
                    handle_client(stream, shared_data_clone);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, shared_data: Arc<Mutex<i32>>) {
    // Read data from the client
    let mut buffer = [0; 512];
    stream.read(&mut buffer).expect("Failed to read from client");

    // Print received data
    let received_data = String::from_utf8_lossy(&buffer);
    println!("Received: {}", received_data);

    // Lock the Mutex to access and modify the shared data safely
    let mut data = shared_data.lock().expect("Failed to acquire lock");

    // Modify the shared data
    *data += 1;

    // Respond to the client with the updated data
    let response = format!("Server received: {}. Shared data is now: {}", received_data, *data);
    stream.write_all(response.as_bytes()).expect("Failed to write to client");

    // Close the connection
    stream.flush().expect("Failed to flush");
}
