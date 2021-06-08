use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:12345")?;

    println!("listening on 127.0.0.1:12345");

    loop {
        let mut socket = match listener.accept() {
            Ok((socket, _)) => socket,
            Err(_) => continue,
        };

        thread::spawn(move || -> Result<(), std::io::Error> {
            let mut buf = [0; 8192];

            loop {
                let n = socket.read(&mut buf)?;

                if n == 0 {
                    return Ok(());
                }

                socket.write_all(&buf[..n])?;
            }
        });
    }
}