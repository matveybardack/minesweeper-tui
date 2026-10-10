use crate::config::Config;
use crate::error::AppError;
use crate::network::models::*;
use route::{Method, RequestBuilder, Response};

use serde::de::DeserializeOwned;
const SESSION_HEADER: &str = "Session-Id";

async fn post(
    cfg: &Config,
    path: &str,
    session_id: Option<&str>,
    body: Option<Vec<u8>>,
) -> Result<Response, AppError> {
    let mut builder = RequestBuilder::new().method(Method::Post).path(path);
    if let Some(id) = session_id {
        builder = builder.header(SESSION_HEADER, id);
    }
    if let Some(body) = body {
        builder = builder
            .header("Content-Type", "application/json")
            .body(body);
    }
    Ok(route::send(&cfg.server_addr, cfg.server_port, builder.build()).await?)
}

pub async fn search_game(cfg: &Config, session_id: &str) -> Result<SearchResponse, AppError> {
    let resp = post(cfg, "/games/minesweeper/search", Some(session_id), None).await?;
    decode(&into_success(resp)?)
}

pub async fn auth(cfg: &Config, nickname: &str) -> Result<SearchResponse, AppError> {
    let body = serde_json::to_vec(&AuthRequest {
        nickname: nickname.to_string(),
    })?;
    let resp = post(cfg, "/auth", None, Some(body)).await?;
    decode(&into_success(resp)?)
}

pub async fn cancel_search(cfg: &Config, session_id: &str) -> Result<(), AppError> {
    let resp = post(cfg, "/game/minesweeper/cancel", Some(session_id), None).await?;
    into_success(resp)?;
    Ok(())
}

pub async fn open_cell(
    cfg: &Config,
    session_id: &str,
    x: u8,
    y: u8,
) -> Result<OpenCellResponse, AppError> {
    let body = serde_json::to_vec(&OpenCellRequest { x, y })?;
    let resp = post(
        cfg,
        format!("/game/minesweeper/{session_id}/cells").as_str(),
        Some(session_id),
        Some(body),
    )
    .await?;
    decode(&into_success(resp)?)
}

pub async fn surrender(cfg: &Config, session_id: &str) -> Result<SurrenderResponse, AppError> {
    let resp = post(
        cfg,
        format!("/game/minesweeper/{session_id}/surrender").as_str(),
        Some(session_id),
        None,
    )
    .await?;
    let surr: SurrenderResponse = decode(&into_success(resp)?)?;

    let ans = match surr.status {
        GameStatus::Lost => Ok(surr),
        _ => Err(AppError::WrongContent {
            content: (surr.status.to_string()),
        }),
    };

    ans
}

fn into_success(resp: Response) -> Result<Response, AppError> {
    if (200..300).contains(&resp.status) {
        return Ok(resp);
    }
    match serde_json::from_slice::<ApiError>(&resp.body) {
        Ok(err) => Err(AppError::Api(err)),
        Err(_) => Err(AppError::UnexpectedError {
            status: resp.status,
        }),
    }
}

fn decode<T: DeserializeOwned>(resp: &Response) -> Result<T, AppError> {
    Ok(serde_json::from_slice(&resp.body)?)
}
