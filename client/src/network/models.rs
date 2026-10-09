use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameStatus {
    Playing,
    Won,
    Lost,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub value: u8,
}

/// Raw server response, parsing code into error
/// # See Also
/// ['ErrorCode'](crate::network::error::ErrorCode)
#[derive(Debug, Clone, Deserialize)]
pub struct ApiError {
    pub code: u16,
    pub message: String,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct BoardSize {
    pub width: u8,
    pub height: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthResponse {
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthRequest {
    pub nickname: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchResponse {
    pub game_id: String,
    pub board_size: BoardSize,
    pub known_cells: Vec<Cell>,
    pub timer_seconds: u32,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct OpenCellRequest {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OpenCellResponse {
    pub cells: Vec<Cell>,
    pub status: GameStatus,
    pub seconds_left: u32,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct SurrenderResponse {
    pub status: GameStatus,
    pub seconds_left: u32,
}
