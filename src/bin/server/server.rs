use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use colored::Colorize;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{self, Receiver, Sender};

async fn on_recieve(buffer: &String, tx: &Sender<String>, address: &SocketAddr) {
    tx.send(buffer.clone()).unwrap();

    println!(
        "{}\"{}\" ({}:{})",
        "<DATA> ".bold().cyan(),
        buffer.trim_end(),
        address.ip(),
        address.port()
    );
}

async fn server(
    tx: Sender<String>,
    mut rx: Receiver<String>,
    mut socket: TcpStream,
    address: SocketAddr,
) {
    let (reader, mut writer) = socket.split();
    let mut socket_reader = BufReader::new(reader);

    let mut buffer = String::new();

    loop {
        tokio::select! {
            bytes_recv = socket_reader.read_line(&mut buffer) => {
                if bytes_recv.unwrap() == 0 {
                    break
                }

                on_recieve(&buffer, &tx, &address).await;

                buffer.clear();
            }

            data = rx.recv() => {
                let msg = data.unwrap();
                writer.write_all(msg.as_bytes()).await.unwrap();
            }
        }
    }
}

pub async fn run(port: u16) {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);

    let listener = TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| panic!("failed to start the TCP listener: {}", e));

    let (tx, _) = broadcast::channel::<String>(25);

    loop {
        let (socket, address) = listener
            .accept()
            .await
            .unwrap_or_else(|e| panic!("failed to accept a connection to the TCP listener: {}", e));

        let tx = tx.clone();
        let rx = tx.subscribe();

        tokio::spawn(async move { server(tx, rx, socket, address).await });
    }
}
