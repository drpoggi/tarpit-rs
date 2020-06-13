use tokio::io::{AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::time::delay_for;
use rand::prelude::*;
use uuid::Uuid;

use std::env;
use std::error::Error;
use std::time::{Duration, Instant};

#[macro_use] extern crate log;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let env = Env::default()
        .filter_or("LOG_LEVEL", "trace")
        .filter_or("RUST_LOG", "tarpit_rs");

    env_logger::init_from_env(env);
    
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
    info!("Listening on: {}", addr);
    

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let peer = socket.peer_addr().unwrap();
            let start = Instant::now();
            let tag = Uuid::new_v4();
            
            info!("new connection {} from {}", tag, peer.to_string());

            loop {                
                delay_for(Duration::new(10, 0)).await;
          
                let id: u32 = random();
                if let Err(_err) = socket.write_all(format!("{}\r\n", id).as_bytes()).await {
                    info!("lost {} -- {} -- duration {}secs", tag, peer.to_string(), start.elapsed().as_secs());
                    return;
                }
                socket.flush().await.expect(&format!("error flushing {}", tag).to_string());
                
            }
        });
    }
}
