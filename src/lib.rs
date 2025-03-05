use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::fs;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string("static/index.html").unwrap_or_else(|_| {
        fs::read_to_string("static/error.html")
            .expect("Error page not found!")
    });

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}",
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn run_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server running on http://127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpStream;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_server_starts() {
        thread::spawn(|| {
            run_server();
        });

        thread::sleep(Duration::from_millis(100));

        match TcpStream::connect("127.0.0.1:7878") {
            Ok(_) => assert!(true),
            Err(e) => panic!("Failed to connect to server: {}", e),
        }
    }

    #[test]
    fn test_connection_receives_response() {
        thread::spawn(|| {
            run_server();
        });

        thread::sleep(Duration::from_millis(100));

        if let Ok(mut stream) = TcpStream::connect("127.0.0.1:7878") {
            let request = "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
            stream.write_all(request.as_bytes()).unwrap();

            let mut response = String::new();
            stream.read_to_string(&mut response).unwrap();

            assert!(response.contains("HTTP/1.1 200 OK"));
            assert!(response.contains("Content-Type: text/html"));
        } else {
            panic!("Failed to connect to server");
        }
    }
}