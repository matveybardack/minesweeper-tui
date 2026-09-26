use crate::{Headers, Method};

/// HTTP-request
/// # Fields
/// - method: GET, POST
/// - path: url of api
/// - headers:
/// - body:
/// # See Also
/// [Headers], [Method]
#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: Headers,
    pub body: Vec<u8>,
}

impl Request {
    /// Serialize request into raw bytes HTTP/1.1 for socket. host, port only for header
    ///
    /// Autocomplete:
    /// - Host: host:port
    /// - Content-Length: <len>
    /// - Connection: close
    pub fn to_bytes(&self, host: &str, port: u16) -> Vec<u8> {
        let mut headers = self.headers.clone();

        if headers.get("Host").is_none() {
            headers.insert("Host", format!("{host}:{port}"));
        }
        if !self.body.is_empty() && headers.get("Content-Length").is_none() {
            headers.insert("Content-Length", self.body.len().to_string());
        }
        if headers.get("Connection").is_none() {
            headers.insert("Connection", "close");
        }

        let mut out = Vec::new();
        out.extend_from_slice(format!("{} {} HTTP/1.1\r\n", self.method, self.path).as_bytes());
        for (name, value) in headers.iter() {
            out.extend_from_slice(format!("{name}: {value}\r\n").as_bytes());
        }
        out.extend_from_slice(b"\r\n");
        out.extend_from_slice(&self.body);
        out
    }
}

/// Fluent-builder for `Request`.
/// # See Also
/// [Request]
pub struct RequestBuilder {
    method: Method,
    path: String,
    headers: Headers,
    body: Vec<u8>,
}

impl RequestBuilder {
    pub fn new() -> Self {
        Self {
            method: Method::Get,
            path: String::from("/"),
            headers: Headers::new(),
            body: Vec::new(),
        }
    }

    pub fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = path.into();
        self
    }

    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name, value);
        self
    }

    /// Client should serialize to json before
    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.body = body;
        self
    }

    pub fn build(self) -> Request {
        Request {
            method: self.method,
            path: self.path,
            headers: self.headers,
            body: self.body,
        }
    }
}

impl Default for RequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}
