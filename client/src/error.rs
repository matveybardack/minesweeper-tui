#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("config error: {0}")]
    Config(String),
    // Route(#[from] route::RouteError)
    //Json(#[from] serde_json::Error)
    //Api(ApiError)
}
