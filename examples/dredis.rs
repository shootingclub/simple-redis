use anyhow::Result;
use simple_redis::BUF_SIZE;
use std::io;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 6379);
    redis_sever(std::net::SocketAddr::V4(addr)).await?;
    Ok(())
}

async fn process_redis_conn(mut stream: TcpStream, raddr: SocketAddr) -> Result<()> {
    loop {
        // wait for readable 数据是否全部准备好
        stream.readable().await?;
        let mut buf = Vec::with_capacity(BUF_SIZE);

        // Try to read data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                info!("read {} bytes", n);
                let line = String::from_utf8_lossy(&buf);
                info!("{:?}", line);
                stream.write_all(b"+OK---\r\n").await?;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
    warn!("Connection {} closed", raddr);
    Ok(())
}

async fn redis_sever(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    //bind server addr
    let listener = TcpListener::bind(addr).await?;
    loop {
        // accept connection
        // 在 Rust 的 Tokio 库中，listener.accept() 方法本身是异步的，不是阻塞的。
        // 这意味着当你调用 listener.accept() 时，它不会阻塞当前线程，而是立即返回一个 Future，
        // 这个 Future 在客户端连接被接受时解析为相应的 Stream。
        let (stream, addr) = listener.accept().await?;
        info!("Accepted connection from: {:?}", addr);
        tokio::spawn(async move {
            if let Err(e) = process_redis_conn(stream, addr).await {
                warn!("Error processing conn with {}: {:?}", addr, e);
            }
        });
    }
}

async fn _redis_client(addr: SocketAddr, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 连接到服务器
    let mut stream = TcpStream::connect(addr).await?;
    // 发送数据到服务器
    stream.write_all(message.as_ref()).await?;
    // 从服务器接收数据
    let mut buffer = vec![0u8; 1024];
    let n_bytes = stream.read(&mut buffer).await?;
    let response = String::from_utf8_lossy(&buffer[..n_bytes]);
    println!("Received from server: {}", response);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[tokio::test]
    async fn test_process_redis_conn() -> Result<(), Box<dyn std::error::Error>> {
        let content = "Hello, world!";
        let addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 6379);
        _redis_client(SocketAddr::V4(addr), content).await?;
        Ok(())
    }
}
