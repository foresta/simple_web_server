use std::thread;
use std::net::{ TcpListener, TcpStream };
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream)
                });
            }
            Err(_) => { panic!("connection failed") }
        }
    }
}

fn handle_client(stream: TcpStream) {

    let mut stream = BufReader::new(stream);
    
    let mut first_line = String::new();
    if let Err(err) = stream.read_line(&mut first_line) {
        panic!("error during receive a line: {}", err);
    }

    let mut params = first_line.split_whitespace();
    let method = params.next().unwrap();
    let path = params.next().unwrap();
    let protocol = params.next().unwrap();

    println!("[METHOD]: {}", method);
    println!("[PATH]: {}", path);
    println!("[PROTOCOL]: {}", protocol);
}
