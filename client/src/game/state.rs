use crossterm::cursor;

use crate::error::AppError;
use crate::game::board::{self, Board};
use crate::game::timer::Timer;
use crate::network::models::GameStatus;
use crate::network::models::{BoardSize, OpenCellResponse, SearchResponse};
use std::time::Instant;

/// Reason of game ending
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Won,
    LostByOpponent,
    LostBySurrender,
    LostByTimeout,
    LostByMine,
}

/// Data about game
pub struct Playing {
    pub game_id: String,
    pub board: Board,
    pub timer: Timer,
    pub cursor: (u8, u8),
}

pub enum GameState {
    Menu,
    Searching { cancel_requested: bool },
    Playing(Playing),
    Finished(Outcome),
}

impl GameState {
    pub fn start_search(&mut self) {
        match &self {
            &GameState::Menu | &GameState::Finished(_) => {
                *self = GameState::Searching {
                    cancel_requested: false,
                }
            }
            _ => (),
        }
    }

    pub fn request_cancel(&mut self) {
        match self {
            GameState::Searching { cancel_requested } => *cancel_requested = true,
            _ => (),
        }
    }

    pub fn finish(&mut self, outcome: Outcome) {
        *self = GameState::Finished(outcome);
    }

    pub fn search_finished_with(&mut self, data: SearchResponse) -> Result<(), AppError> {
        let board = Board::new(data.board_size, &data.known_cells)?;
        let timer = Timer::new_at(data.timer_seconds, Instant::now());
        let play = Playing {
            game_id: data.game_id,
            board,
            timer,
            cursor: (0u8, 0u8),
        };

        *self = GameState::Playing(play);
        Ok(())
    }

    //pub fn apply_move(
    ////&mut self,
    ////cell_resp: &OpenCellResponse,
    ////now: Instant,
    //) -> Result<(), AppError> {
    ////match
    //}
}

pub fn outcome_of(resp: &OpenCellResponse) -> Option<Outcome> {
    match resp.status {
        GameStatus::Playing => None,
        GameStatus::Won => Some(Outcome::Won),
        GameStatus::Lost if resp.cells.iter().any(|c| c.value == 9) => Some(Outcome::LostByMine),
        GameStatus::Lost => Some(Outcome::LostByOpponent),
    }
}
