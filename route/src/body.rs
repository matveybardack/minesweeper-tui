use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::{Headers, RouteError};

/// Read response body `Content-Length`.
/// # Params
/// - stream:
/// - headers:
/// - leftover: "after words"
pub async fn read_body(
    stream: &mut TcpStream,
    headers: &Headers,
    leftover: Vec<u8>,
) -> Result<Vec<u8>, RouteError> {
    let content_length: usize = match headers.get("Content-Length") {
        Some(v) => v.trim().parse().unwrap_or(0),
        None => 0,
    };

    if content_length == 0 {
        return Ok(Vec::new());
    }

    let mut body = leftover;
    body.reserve(content_length.saturating_sub(body.len()));

    while body.len() < content_length {
        let mut chunk = [0u8; 512];
        let n = stream.read(&mut chunk).await?;
        if n == 0 {
            return Err(RouteError::UnexpectedEof);
        }
        body.extend_from_slice(&chunk[..n]);
    }

    body.truncate(content_length);
    Ok(body)
}
