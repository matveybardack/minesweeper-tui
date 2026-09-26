use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::RouteError;

pub async fn connect(host: &str, port: u16) -> Result<TcpStream, RouteError> {
    let stream = TcpStream::connect((host, port)).await?;
    Ok(stream)
}

pub async fn write_request(stream: &mut TcpStream, bytes: &[u8]) -> Result<(), RouteError> {
    stream.write_all(bytes).await?;
    Ok(())
}

/// Raw bytes of headers with "after words" (after end "\r\n\r\n", maybe body)
pub struct HeaderReadResult {
    pub header_bytes: Vec<u8>,
    pub leftover: Vec<u8>,
}

/// Read from socket to "\r\n\r\n" and "arter words"
pub async fn read_headers(stream: &mut TcpStream) -> Result<HeaderReadResult, RouteError> {
    let mut buf = Vec::new();
    let mut chunk = [0u8; 512];

    loop {
        let n = stream.read(&mut chunk).await?;
        if n == 0 {
            return Err(RouteError::UnexpectedEof);
        }
        buf.extend_from_slice(&chunk[..n]);

        if let Some(pos) = find_header_terminator(&buf) {
            let header_bytes = buf[..pos].to_vec();
            let leftover = buf[pos + 4..].to_vec();
            return Ok(HeaderReadResult {
                header_bytes,
                leftover,
            });
        }
    }
}

/// Finds end of header
fn find_header_terminator(buf: &[u8]) -> Option<usize> {
    buf.windows(4).position(|w| w == b"\r\n\r\n")
}
