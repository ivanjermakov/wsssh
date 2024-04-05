use std::io::{Read, Write};

use anyhow::{Context, Result};
use ssh2::Session;
use tokio::net::TcpStream;

mod hex;

#[tokio::main]
async fn main() -> Result<()> {
    let tcp = TcpStream::connect("127.0.0.1:2222")
        .await
        .context("tcp connection")?;
    let mut sess = Session::new().context("create session")?;
    sess.set_tcp_stream(tcp);
    sess.handshake().context("handshake")?;
    sess.userauth_password("ivan", "oneone")
        .context("authentication")?;
    assert!(sess.authenticated());
    let mut ch = sess.channel_session()?;
    ch.request_pty("xterm", None, Some((100, 40, 0, 0)))
        .context("pty")?;
    ch.shell().context("shell")?;

    ch.write_all(b"echo hello\n")?;
    ch.write_all(b"exit\n")?;

    loop {
        let mut buf = [0u8; 1024];

        match ch.read(&mut buf) {
            Ok(0) => break,
            Ok(c) => {
                println!("{}", hex::hex(&buf[0..c]));
                // print!("{}", from_utf8(&buf[0..c])?);
            }
            Err(_) => break,
        }
    }

    ch.send_eof()?;
    ch.wait_eof()?;
    ch.close()?;
    ch.wait_close()?;

    Ok(())
}
