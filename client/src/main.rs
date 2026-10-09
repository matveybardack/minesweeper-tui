pub mod config;
pub mod error;
pub mod game;
pub mod network;
pub mod ui;

use crate::error::AppError;
use crossterm::{
    execute,
    terminal::{
        EnableLineWrap, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use std::io;
use ui::menu_screen::App;

fn main() -> Result<(), AppError> {
    let config = config::load()?; // до входа в alternate screen: ошибку конфига должно быть видно

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    let result = ratatui::run(|terminal| rt.block_on(App::new(config).run(terminal)));
    result
}
