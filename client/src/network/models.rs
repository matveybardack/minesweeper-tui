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
    pub x: u8,
    pub y: u8,
    pub value: u8,
}

/// Raw server response, parsing code into error
/// # See Also
/// [`ErrorCode`](crate::network::error::ErrorCode)
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
    pub x: u8,
    pub y: u8,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_open_cell_response() {
        let json = r#"{"cells":[{"x":0,"y":1,"value":9}],"status":"lost","seconds_left":42}"#;
        let r: OpenCellResponse = serde_json::from_str(json).unwrap();
        assert_eq!(r.status, GameStatus::Lost);
        assert_eq!(r.cells[0].value, 9);
    }
}
