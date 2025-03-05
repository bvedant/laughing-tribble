use std::net::TcpStream;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

#[test]
fn test_server_integration() {
    // Start the server from our library
    thread::spawn(|| {
        laughing_tribble::run_server();
    });

    // Give it a moment to start
    thread::sleep(Duration::from_millis(100));

    // Test multiple concurrent connections
    let mut handles = vec![];
    
    for _ in 0..3 {
        handles.push(thread::spawn(|| {
            if let Ok(mut stream) = TcpStream::connect("127.0.0.1:7878") {
                let request = "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
                stream.write_all(request.as_bytes()).unwrap();

                let mut response = String::new();
                stream.read_to_string(&mut response).unwrap();

                assert!(response.contains("HTTP/1.1 200 OK"));
            } else {
                panic!("Failed to connect to server");
            }
        }));
    }

    // Wait for all test threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}