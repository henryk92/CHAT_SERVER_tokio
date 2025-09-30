use futures::SinkExt;
use tokio::net::TcpListener;
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};

#[tokio::main]
// main을 동기 함수로 만들어 비동기 코드를 작동하는 매크로
async fn main() -> anyhow::Result<()> {
    // 반환값을 anyhow::Result<()>을 사용해 오류(?) 처리를 간략화
    let server = TcpListener::bind("127.0.0.1:6684").await?;
    // 127.0.0.1:6684 소켓 바인드 (Future 반환후 await 으로 대기)
    loop {
        let (mut tcp, _) = server.accept().await?;
        // accept()는 Future의 Result<(TcpStream, SocketAddr)> 값을 반환
        println!("Connect");
        let mut lines = Framed::new(tcp, LinesCodec::new());
        while let Some(line) = lines.next().await {
            let mut line = line?;
            println!("{line}");
            line.push_str(" ♥");
            lines.send(line).await?;
        }
    }
}