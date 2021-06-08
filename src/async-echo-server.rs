use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:12345").await?;

    println!("listening on 127.0.0.1:12345");

    tokio::spawn(async move {
        loop {
            let mut socket = match listener.accept().await {
                Ok((socket, _)) => socket,
                Err(_) => continue,
            };

            tokio::spawn(async move {
                let mut buf = vec![0; 8192];

                loop {
                    let n = socket.read(&mut buf).await?;

                    if n == 0 {
                        return Ok::<_, std::io::Error>(());
                    }

                    socket.write_all(&buf[..n]).await?;
                }
            });
        }
    }).await?;

    Ok(())
}