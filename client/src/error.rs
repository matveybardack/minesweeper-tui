use crate::network::models::ApiError;
use std::io::Error;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("config error: {0}")]
    Config(String),
    // Route(#[from] route::RouteError)
    //Json(#[from] serde_json::Error)
    #[error("api error {}: {}", .0.code, .0.message)]
    Api(ApiError),
    #[error("io error: {0}")]
    Io(#[from] Error),
}
