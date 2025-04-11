use std::io::{self, Write};
use std::net::TcpStream;
use std::thread;
use std::io::Read;

const MSG_SIZE: usize = 32;

fn main() {
    let mut client = TcpStream::connect("127.0.0.1:6000").expect("Failed to connect to server");
    client.set_nonblocking(true).expect("Failed to set non-blocking");

    let mut client_clone = client.try_clone().expect("Failed to clone client");

    // Spawn a thread to handle incoming messages
    thread::spawn(move || {
        let mut buff = vec![0; MSG_SIZE];
        loop {
            match client_clone.read_exact(&mut buff) {
                Ok(_) => {
                    let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                    let msg = String::from_utf8(msg).expect("Invalid utf8 message");
                    println!("{}", msg);
                },
                Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => (),
                Err(_) => break,
            }
            buff = vec![0; MSG_SIZE];
        }
    });

    // Main thread handles user input
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let mut buff = input.trim().to_string().into_bytes();
        buff.resize(MSG_SIZE, 0);
        client.write_all(&buff).expect("Failed to write to server");
    }
} 