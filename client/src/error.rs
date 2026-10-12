use crate::network::models::ApiError;
use std::mem::discriminant;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("config error: {0}")]
    Config(String),
    #[error("route error: {0}")]
    Route(#[from] route::RouteError),
    #[error("json parse error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("api error {}: {}", .0.code, .0.message)]
    Api(ApiError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("unexpected error status: {status}")]
    UnexpectedError { status: u16 },
    #[error("server returned wrong content {content}")]
    WrongContent { content: String },
}

impl PartialEq for AppError {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}
