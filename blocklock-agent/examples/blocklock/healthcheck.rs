use tokio::io::AsyncWriteExt;

pub async fn healthcheck(address: &str, port: u16) -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind((address, port)).await?;
    println!("Healthcheck listening on {:?}", listener.local_addr()?);

    loop {
        let (mut socket, _) = listener.accept().await?;
        let _ = socket
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 7\r\n\r\nhealthy")
            .await;
    }
}
