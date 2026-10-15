use crate::error::AppError;
use crate::game::board::Board;
use crate::game::timer::Timer;
use crate::network::models::{GameStatus, OpenCellResponse, SearchResponse};
use std::time::Instant;

use crate::game::state::outcome::Outcome;
use crate::game::state::playing::Playing;

#[derive(Debug)]
pub enum GameState {
    Menu,
    Searching { cancel_requested: bool },
    Playing(Playing),
    Finished(Outcome),
}

impl GameState {
    /// Getter for cancel searchinf flag
    pub fn cancel_requested(&self) -> bool {
        matches!(
            self,
            GameState::Searching {
                cancel_requested: true
            }
        )
    }

    /// Starting search the opponent
    /// # Screens from can
    /// - Finished
    /// - Menu
    /// # Next screen
    /// - Searching
    pub fn start_search(&mut self) {
        if matches!(self, GameState::Finished(_) | GameState::Menu) {
            *self = GameState::Searching {
                cancel_requested: false,
            }
        }
    }

    /// Cancel searching the opponent by client
    /// # Screens from can
    /// - Searching
    /// # Next screen
    /// - Searching
    pub fn request_cancel(&mut self) {
        if let GameState::Searching { cancel_requested } = self {
            *cancel_requested = true
        }
    }

    /// Finish the game (won/lost)
    /// # Screens from can
    /// - Playing
    /// # Next screen
    /// - Finished
    pub fn finish(&mut self, outcome: Outcome) {
        if matches!(self, GameState::Playing(_)) {
            *self = GameState::Finished(outcome);
        }
    }

    /// Return to menu (only innactive game)
    /// # Screens from can
    /// - Finished
    /// - Searching
    /// # Next screen
    /// - Menu
    pub fn to_menu(&mut self) {
        if matches!(self, GameState::Searching { .. } | GameState::Finished(_)) {
            *self = GameState::Menu;
        }
    }

    /// Open/close surrender dialog
    /// # Screens from can
    /// - Playing
    /// # Next screen
    /// - Playing (on backscreen)
    pub fn toggle_surrender_dialog(&mut self, flag: bool) {
        if let GameState::Playing(game) = self {
            game.confirm_surrender = flag
        }
    }

    /// Stop searching the opponent by server
    /// time from causer
    /// # Screens from can
    /// - Searching
    /// # Next screen
    /// - Playing (without error)
    /// - Searching (server data error)
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
            confirm_surrender: false,
        };

        *self = GameState::Playing(play);
        Ok(())
    }

    /// Stop searching the opponent by server
    /// # Screens from can
    /// - Searching
    /// # Next screen
    /// - Playing (without error)
    /// - Searching (server data error)
    pub fn search_finished_with(&mut self, data: SearchResponse) -> Result<(), AppError> {
        let GameState::Searching { .. } = self else {
            return Ok(());
        };

        let now = Instant::now();
        let board = Board::new(data.board_size, &data.known_cells)?;
        let timer = Timer::new_at(data.timer_seconds, now);
        let play = Playing {
            game_id: data.game_id,
            board,
            timer,
            cursor: (0u8, 0u8),
            confirm_surrender: false,
        };

        *self = GameState::Playing(play);
        Ok(())
    }

    /// Open new cells from server response
    /// time by causer
    /// # Screens from can
    /// - Playing
    /// # Next screen
    /// - Playing
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

    /// Open new cells from server response
    /// # Screens from can
    /// - Playing
    /// # Next screen
    /// - Playing
    pub fn apply_move(&mut self, resp: &OpenCellResponse) -> Result<(), AppError> {
        let GameState::Playing(game) = self else {
            return Ok(());
        };

        let now = Instant::now();
        let mine_opened = game.board.apply(&resp.cells)?;
        let outcome = outcome_of(resp.status, mine_opened, resp.seconds_left == 0)?;

        game.timer.reset_at(resp.seconds_left, now);
        if let Some(outcome) = outcome {
            *self = GameState::Finished(outcome);
        }
        Ok(())
    }
}

/// DeterMINE the reason of lost or won
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
    use crate::network::models::BoardSize;
    use crate::network::models::Cell;

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

    #[test]
    fn timer_update() {
        let now = Instant::now();
        let mut game = GameState::Searching {
            cancel_requested: false,
        };
        let search_resp = SearchResponse {
            game_id: "0".to_string(),
            board_size: BoardSize {
                width: 1,
                height: 1,
            },
            known_cells: vec![Cell {
                x: 0,
                y: 0,
                value: 5,
            }],
            timer_seconds: 10,
        };

        game.search_finished_with_at(search_resp, now).unwrap();

        if let GameState::Playing(playing) = game {
            assert_eq!(10, playing.timer.remaining_secs_at(now))
        } else {
            panic!("Wrong GameState type")
        }
    }
}
