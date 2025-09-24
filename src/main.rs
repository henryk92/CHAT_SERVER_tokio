use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};

#[tokio::main]
// main을 동기 함수로 만들어 비동기 코드를 작동하는 매크로
async fn main() -> anyhow::Result<()> {
    let server = TcpListener::bind("127.0.0.1:42069").await?;
    let (mut tcp, _) = server.accept().await?;
    let mut buffer = [0u8; 16];
    loop {
        let n = tcp.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        let _ = tcp.write(&buffer[..n]).await?;
    }
    Ok(())
}