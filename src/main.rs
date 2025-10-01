use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LinesCodec};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = TcpListener::bind("127.0.0.1:6684").await?;
    // 127.0.0.1:6684 소켓 바인드

    loop {
        let (mut tcp, _) = server.accept().await?;
        // accept()는 Future의 Result<(TcpStream, SocketAddr)> 값을 반환
        println!("Connect");

        tokio::spawn(async move {
            let mut lines = Framed::new(tcp, LinesCodec::new());
            // Framed를 이용하여 저수준 바이트 스트림을 고수준 메시지 단위 스트림으로 변경 (LinesCodec 코딩방식 사용)
            // LinesCodec
            // - read: TCP에서 바이트를 읽다가 \n을 만나면 거기서 끊어서 String으로 반환
            // - write: String을 바이트로 바꾸고, 자동으로 \n을 붙여서 TCP에 씀

            let (mut sink, mut stream) = lines.split::<String>();
            // lines를 쓰기(Sink) / 읽기(Stream) 로 분리.

            while let Some(line) = stream.next().await {
                // 비동기적으로 .next().await를 호출하면 String 메시지를 한 줄씩 얻을 수 있음
                let mut line = match line {
                    Ok(l) => l,
                    Err(e) => {
                        eprintln!("Error reading line: {:?}", e);
                        break;
                    }
                };
                println!("{line}");
                line.push_str(" return");
                // 읽은 메세지 뒤에 문자 붙임
                if let Err(e) = sink.send(line).await {
                    eprintln!("Error sending line: {:?}", e);
                    break;
                }
            }
        });
    }
}