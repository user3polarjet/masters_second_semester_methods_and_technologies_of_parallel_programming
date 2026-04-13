use std::collections::HashSet;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Mutex, broadcast};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let (tx, _rx) = broadcast::channel::<(String, std::net::SocketAddr)>(10);

    // Shared, thread-safe collection of active nicknames
    let active_users = Arc::new(Mutex::new(HashSet::new()));

    println!("Server started on 127.0.0.1:8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        let active_users = Arc::clone(&active_users);

        tokio::spawn(async move {
            let (reader, mut writer) = socket.into_split();
            let mut reader = BufReader::new(reader);
            let mut nickname = String::new();

            // 1. Registration Phase
            if reader.read_line(&mut nickname).await.unwrap_or(0) == 0 {
                return;
            }
            let nickname = nickname.trim().to_string();

            {
                let mut users = active_users.lock().await;
                if users.contains(&nickname) {
                    let _ = writer.write_all(b"TAKEN\n").await;
                    return; // Reject and drop connection
                }
                users.insert(nickname.clone());
            }

            // Acknowledge successful registration
            let _ = writer.write_all(b"OK\n").await;
            let _ = tx.send((format!("*** {} joined the chat ***\n", nickname), addr));

            // 2. Main Message Loop
            let mut line = String::new();
            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap_or(0) == 0 {
                            break; // Client disconnected
                        }
                        // Format message with the sender's nickname
                        let msg = format!("{}: {}", nickname, line);
                        let _ = tx.send((msg, addr));
                        line.clear();
                    }
                    result = rx.recv() => {
                        if let Ok((msg, sender_addr)) = result {
                            if addr != sender_addr {
                                let _ = writer.write_all(msg.as_bytes()).await;
                            }
                        }
                    }
                }
            }

            // 3. Cleanup Phase
            active_users.lock().await.remove(&nickname);
            let _ = tx.send((format!("*** {} left the chat ***\n", nickname), addr));
        });
    }
}
