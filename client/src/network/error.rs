use crate::network::models::ApiError;

/// Server error code response
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    InvalidRequest,      // 1000
    SessionInvalid,      // 1001
    NicknameTaken,       // 1002
    SearchAlreadyActive, // 2001
    SearchNotActive,     // 2002
    SearchEnded,         // 2003
    GameNotFound,        // 3001
    CellOutOfBounds,     // 3002
    InternalError,       // 5000
    Unknown(u16),
}

impl From<u16> for ErrorCode {
    fn from(code: u16) -> Self {
        match code {
            1000 => Self::InvalidRequest,
            1001 => Self::SessionInvalid,
            1002 => Self::NicknameTaken,
            2001 => Self::SearchAlreadyActive,
            2002 => Self::SearchNotActive,
            2003 => Self::SearchEnded,
            3001 => Self::GameNotFound,
            3002 => Self::CellOutOfBounds,
            5000 => Self::InternalError,
            other => Self::Unknown(other),
        }
    }
}

impl ApiError {
    pub fn kind(&self) -> ErrorCode {
        self.code.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_known_code() {
        let e: ApiError = serde_json::from_str(r#"{"code":3001,"message":"x"}"#).unwrap();
        assert_eq!(e.kind(), ErrorCode::GameNotFound);
    }

    #[test]
    fn parses_unknown_code() {
        let e: ApiError = serde_json::from_str(r#"{"code":4000,"message":"x"}"#).unwrap();
        assert_eq!(e.kind(), ErrorCode::Unknown(4000));
    }
}
