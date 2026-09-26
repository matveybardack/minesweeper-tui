use std::fmt::{Display, Formatter, Result, write};

/// HTTP method of request
#[derive(Debug, Clone)]
pub enum Method {
    Get,
    Post,
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let out = match self {
            Method::Get => "GET",
            Method::Post => "POST",
        };
        write!(f, "{out}")
    }
}
