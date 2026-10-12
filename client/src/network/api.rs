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

pub async fn auth(cfg: &Config, nickname: &str) -> Result<String, AppError> {
    let body = serde_json::to_vec(&AuthRequest {
        nickname: nickname.to_string(),
    })?;
    let resp = post(cfg, "/auth", None, Some(body)).await?;
    let resp: AuthResponse = decode(&into_success(resp)?)?;

    Ok(resp.session_id)
}

pub async fn cancel_search(cfg: &Config, session_id: &str) -> Result<(), AppError> {
    let resp = post(
        cfg,
        "/games/minesweeper/search/cancel",
        Some(session_id),
        None,
    )
    .await?;
    into_success(resp)?;
    Ok(())
}

pub async fn open_cell(
    cfg: &Config,
    session_id: &str,
    game_id: &str,
    x: u8,
    y: u8,
) -> Result<OpenCellResponse, AppError> {
    let body = serde_json::to_vec(&OpenCellRequest { x, y })?;
    let resp = post(
        cfg,
        format!("/games/minesweeper/{game_id}/cells").as_str(),
        Some(session_id),
        Some(body),
    )
    .await?;
    decode(&into_success(resp)?)
}

pub async fn surrender(
    cfg: &Config,
    session_id: &str,
    game_id: &str,
) -> Result<SurrenderResponse, AppError> {
    let resp = post(
        cfg,
        format!("/games/minesweeper/{game_id}/surrender").as_str(),
        Some(session_id),
        None,
    )
    .await?;
    let surr: SurrenderResponse = decode(&into_success(resp)?)?;

    match surr.status {
        GameStatus::Lost => Ok(surr),
        _ => Err(AppError::WrongContent {
            content: (surr.status.to_string()),
        }),
    }
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

#[cfg(test)]
mod test {
    use route::Headers;

    use crate::network::error::ErrorCode;

    use super::*;

    #[test]
    fn into_success_code_3001() {
        let resp = Response {
            status: 404,
            headers: Headers::new(),
            body: br#"{"code":3001,"message":"x"}"#.to_vec(),
        };

        let kind = into_success(resp).unwrap_err();
        let kind = match kind {
            AppError::Api(err) => err.kind(),
            _ => panic!("Not GameNotFound"),
        };
        assert_eq!(ErrorCode::GameNotFound, kind);
    }

    #[test]
    fn into_success_unknown_code() {
        let resp = Response {
            status: 502,
            headers: Headers::new(),
            body: br#"<html>some</html>"#.to_vec(),
        };

        let kind = into_success(resp).unwrap_err();
        assert_eq!(AppError::UnexpectedError { status: 0 }, kind);
    }

    #[test]
    fn into_success_empty_body() {
        let resp = Response {
            status: 204,
            headers: Headers::new(),
            body: vec![],
        };

        let _ = into_success(resp).unwrap();
    }

    #[test]
    fn decode_authresponse_into_str() {
        let resp = Response {
            status: 204, // should pass
            headers: Headers::new(),
            body: br#"{"session_id":"abc"}"#.to_vec(),
        };

        let kind: AuthResponse = decode(&into_success(resp).unwrap()).unwrap();

        assert_eq!("abc".to_string(), kind.session_id);
    }
}
