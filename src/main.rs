use tokio::io::{AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::time::delay_for;
use rand::prelude::*;

use std::env;
use std::error::Error;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Allow passing an address to listen on as the first argument of this
    // program, but otherwise we'll just set up our TCP listener on
    // 0.0.0.0:2222 for connections.
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:2222".to_string());

    // Next up we create a TCP listener which will listen for incoming
    // connections. This TCP listener is bound to the address we determined
    // above and must be associated with an event loop.
    let mut listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);
    

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let peer = socket.peer_addr().unwrap();
            let start = Instant::now();
            println!("got connection from {}", peer.to_string());

            loop {                
                delay_for(Duration::new(10, 0)).await;
          
                let id: u32 = random();
                if let Err(_err) = socket.write_all(format!("{}\r\n", id).as_bytes()).await {
                    println!("lost connection to {} -- duration {}secs", peer.to_string(), start.elapsed().as_secs());
                    return;
                }
                socket.flush().await.expect("some bad shit");
                
            }
        });
    }
}
