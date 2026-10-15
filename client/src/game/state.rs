use crate::error::AppError;
use crate::game::board::Board;
use crate::game::timer::Timer;
use crate::network::models::GameStatus;
use crate::network::models::{OpenCellResponse, SearchResponse};
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
        if matches!(self, GameState::Playing(_)) {
            *self = GameState::Finished(outcome);
        }
    }

    pub fn to_menu(&mut self) {
        if matches!(self, GameState::Searching { .. } | GameState::Finished(_)) {
            *self = GameState::Menu;
        }
    }

    pub fn search_finished_with_at(
        &mut self,
        data: SearchResponse,
        now: Instant,
    ) -> Result<(), AppError> {
        let GameState::Searching { .. } = self else {
            return Ok(());
        };

        let board = Board::new(data.board_size, &data.known_cells)?;
        let timer = Timer::new_at(data.timer_seconds, now);
        let play = Playing {
            game_id: data.game_id,
            board,
            timer,
            cursor: (0u8, 0u8),
        };

        *self = GameState::Playing(play);
        Ok(())
    }

    pub fn apply_move_at(&mut self, resp: &OpenCellResponse, now: Instant) -> Result<(), AppError> {
        let GameState::Playing(game) = self else {
            return Ok(());
        };

        let mine_opened = game.board.apply(&resp.cells)?;
        let outcome = outcome_of(resp.status, mine_opened, resp.seconds_left == 0)?;

        game.timer.reset_at(resp.seconds_left, now);
        if let Some(outcome) = outcome {
            *self = GameState::Finished(outcome);
        }
        Ok(())
    }
}

fn outcome_of(
    status: GameStatus,
    mine_opened: bool,
    is_expired: bool, // Граничный случай сервера, когда таймер истек во время отправки ответа
) -> Result<Option<Outcome>, AppError> {
    if mine_opened && status != GameStatus::Lost {
        return Err(AppError::WrongContent {
            content: format!("status {status:?} with an opened mine"),
        });
    }
    let ans = match status {
        GameStatus::Playing => None,
        GameStatus::Won => Some(Outcome::Won),
        GameStatus::Lost if mine_opened => Some(Outcome::LostByMine),
        GameStatus::Lost if is_expired => Some(Outcome::LostByTimeout),
        GameStatus::Lost => Some(Outcome::LostByOpponent),
    };

    Ok(ans)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn outcome_mine_won() {
        let status = GameStatus::Won;
        let mine_opened = true;
        let is_expired = false;

        let res = outcome_of(status, mine_opened, is_expired).expect_err("Expect error");

        assert_eq!(
            AppError::WrongContent {
                content: "status Won with an opened mine".to_string()
            },
            res
        );
    }
}
