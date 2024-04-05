use anyhow::{Context, Result};
use ssh2::Session;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<()> {
    let tcp = TcpStream::connect("127.0.0.1:2222").await.context("tcp connection")?;
    let mut sess = Session::new().context("create session")?;
    sess.set_tcp_stream(tcp);
    sess.handshake().context("handshake")?;
    sess.userauth_agent("ivan").context()?;
    assert!(sess.authenticated());
    Ok(())
}
