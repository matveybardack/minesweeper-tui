use crate::game::board::Board;
use crate::game::timer::Timer;

/// Data about game
#[derive(Debug)]
pub struct Playing {
    pub game_id: String,
    pub board: Board,
    pub timer: Timer,
    pub cursor: (u8, u8),
    pub confirm_surrender: bool,
}

impl Playing {
    /// Cursor moving with bounds check
    pub fn move_cursor(&mut self, dx: i8, dy: i8) {
        let max_x = self.board.width.saturating_sub(1) as i16;
        let max_y = self.board.height.saturating_sub(1) as i16;
        self.cursor.0 = (self.cursor.0 as i16 + dx as i16).clamp(0, max_x) as u8;
        self.cursor.1 = (self.cursor.1 as i16 + dy as i16).clamp(0, max_y) as u8;
    }
}
