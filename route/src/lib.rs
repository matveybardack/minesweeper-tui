use tokio::io::AsyncWriteExt;

mod body;
mod connection;
mod error;
mod headers;
mod method;
mod request;
mod response;

pub use error::RouteError;
pub use headers::Headers;
pub use method::Method;
pub use request::{Request, RequestBuilder};
pub use response::Response;

/// HTTP: new tcp connection -> request -> read response -> close connection
/// Connection: close auto in Request::to_bytes).
///
/// # Example
/// ```rust
/// use route::{Method, RequestBuilder, Response};
///
/// # async fn example() -> Result<(), route::RouteError> {
/// let request = RequestBuilder::new()
///     .method(Method::Post)
///     .path("/games/search")
///     .header("Content-Type", "application/json")
///     .body(br#"{"nickname":"player1"}"#.to_vec())
///     .build();
///
/// let response: Response = route::send("127.0.0.1", 8080, request).await?;
/// println!("status: {}", response.status);
/// # Ok(())
/// # }
/// ```
pub async fn send(host: &str, port: u16, request: Request) -> Result<Response, RouteError> {
    let mut stream = connection::connect(host, port).await?;

    let bytes = request.to_bytes(host, port);
    connection::write_request(&mut stream, &bytes).await?;

    let head = connection::read_headers(&mut stream).await?;
    let (status_line, headers) = response::parse_response_head(&head.header_bytes)?;

    let body = body::read_body(&mut stream, &headers, head.leftover).await?;

    stream.shutdown().await?;

    Ok(Response {
        status: status_line.status_code,
        headers,
        body,
    })
}
