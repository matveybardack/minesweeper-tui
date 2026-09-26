use crate::{Headers, RouteError};

/// Head of HTTP-response
#[derive(Debug, Clone)]
pub struct StatusLine {
    pub version: String,
    pub status_code: u16,
    pub reason: String,
}

/// Parse of one-line response
// TODO!(server) по стандарту ответ идее одной строкой (без json), но тут зависит от реализации `null`, **СПРОСИТЬ!**
pub fn parse_status_line(line: &str) -> Result<StatusLine, RouteError> {
    let mut parts = line.splitn(3, ' ');

    let version = parts.next().ok_or(RouteError::InvalidStatusLine)?;
    let status_code = parts
        .next()
        .ok_or(RouteError::InvalidStatusLine)?
        .parse::<u16>()
        .map_err(|_| RouteError::InvalidStatusLine)?;
    let reason = parts.next().unwrap_or("").to_string();

    Ok(StatusLine {
        version: version.to_string(),
        status_code,
        reason,
    })
}

/// Parse head into status and headers
pub fn parse_response_head(bytes: &[u8]) -> Result<(StatusLine, Headers), RouteError> {
    let text = std::str::from_utf8(bytes)?;
    let mut lines = text.split("\r\n");

    let status_line = parse_status_line(lines.next().ok_or(RouteError::InvalidStatusLine)?)?;

    let mut headers = Headers::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let (name, value) = line
            .split_once(':')
            .ok_or_else(|| RouteError::InvalidHeader(line.to_string()))?;
        headers.insert(name.trim(), value.trim());
    }

    Ok((status_line, headers))
}

/// HTTP response
#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    pub headers: Headers,
    pub body: Vec<u8>,
}
