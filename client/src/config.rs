use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_addr: String,
    pub server_port: u16,
    pub player_nickname: String,
}

/// Load all data from enviroment
/// # Env Data
/// - SERVER_ADDR: server address
/// - SERVER_PORT: server port
/// - PLAYER_NICKNAME: nickname of player
/// # See Also
/// [AppError]
pub fn load() -> Result<Config, AppError> {
    dotenvy::dotenv().ok(); // system env without .env

    let server_addr = env_var("SERVER_ADDR")?;
    let server_port = env_var("SERVER_PORT")?
        .parse::<u16>()
        .map_err(|_| AppError::Config("SERVER_PORT must be a valid u16".into()))?;
    let player_nickname = env_var("PLAYER_NICKNAME")?;

    if player_nickname.trim().is_empty() {
        return Err(AppError::Config(format!(
            "player`s nickname must not be empty"
        )));
    }

    Ok(Config {
        server_addr,
        server_port,
        player_nickname,
    })
}

/// Read enviroment data by key with parsing errors into AppError
/// # See Also
/// [AppError]
fn env_var(key: &str) -> Result<String, AppError> {
    std::env::var(key).map_err(|_| AppError::Config(format!("{key} is not set")))
}
