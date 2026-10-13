use crate::{
    error::AppError,
    network::models::{BoardSize, Cell},
};

/// Number of mines around the cell (9: cell conatins a mine)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opened {
    Number(u8), // 0..=8
    Mine,       // value == 9
}

impl TryFrom<u8> for Opened {
    type Error = AppError;
    fn try_from(v: u8) -> Result<Self, AppError> {
        match v {
            0..=8 => Ok(Self::Number(v)),
            9 => Ok(Self::Mine),
            _ => Err(AppError::WrongContent {
                content: format!("wrong cell value {v}"),
            }),
        }
    }
}

/// Game cell state
/// Flagged - the inner state of client, not for server
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Closed,
    Flagged, // only on client`s view
    Open(Opened),
}

/// Game board
#[derive(PartialEq, Eq)]
pub struct Board {
    pub width: u8,
    pub height: u8,
    cells: Vec<CellState>,
}

impl Board {
    pub fn new(size: BoardSize, known: &[Cell]) -> Result<Board, AppError> {
        let mut board: Self = Self {
            width: size.width,
            height: size.height,
            cells: vec![CellState::Closed; size.width as usize * size.height as usize],
        };

        board.apply(known)?;
        Ok(board)
    }

    /// Open cells on the board, atomic operation
    pub fn apply(&mut self, cells: &[Cell]) -> Result<(), AppError> {
        let mut updates = Vec::with_capacity(cells.len());
        for c in cells {
            let idx = self.index(c.x, c.y).ok_or_else(|| AppError::WrongContent {
                content: format!("cell {} {} is out of bounds", c.x, c.y),
            })?;
            updates.push((idx, Opened::try_from(c.value)?));
        }

        for (idx, state) in updates {
            self.cells[idx] = CellState::Open(state);
        }

        Ok(())
    }

    pub fn index(&self, x: u8, y: u8) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y as usize * self.width as usize + x as usize)
        } else {
            None
        }
    }

    /// Mark the cell as `Flagged`
    /// >UI cannot open the flagged one
    pub fn toggle_flag(&mut self, x: u8, y: u8) {
        let Some(idx) = self.index(x, y) else { return };
        match self.cells[idx] {
            CellState::Closed => self.cells[idx] = CellState::Flagged,
            CellState::Flagged => self.cells[idx] = CellState::Closed,
            _ => (),
        }
    }

    pub fn get(&self, x: u8, y: u8) -> Option<CellState> {
        self.index(x, y).map(|i| self.cells[i])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_mine_number() {
        let mine = 6;
        assert_eq!(Ok(Opened::Number(6)), Opened::try_from(mine));
    }

    #[test]
    fn parse_mine() {
        let mine = 9;
        assert_eq!(Ok(Opened::Mine), Opened::try_from(mine));
    }

    #[test]
    fn parse_unknown_mine_number() {
        let mine = 32;
        assert_eq!(
            Err(AppError::WrongContent {
                content: "32".to_string()
            }),
            Opened::try_from(mine)
        );
    }

    #[test]
    fn new_board() {
        let size = BoardSize {
            width: 3,
            height: 3,
        };

        let cell_mine = Cell {
            x: 1,
            y: 1,
            value: 9,
        };

        let cell_open = Cell {
            x: 0,
            y: 0,
            value: 8,
        };

        let board = Board::new(size, &vec![cell_mine, cell_open]).unwrap();

        assert_eq!(Some(CellState::Open(Opened::Number(8))), board.get(0, 0));
        assert_eq!(Some(CellState::Open(Opened::Mine)), board.get(1, 1));
        assert_eq!(Some(CellState::Closed), board.get(2, 2));
    }

    #[test]
    fn apply_opens_flag() {
        let size = BoardSize {
            width: 1,
            height: 1,
        };

        let cell_open = Cell {
            x: 0,
            y: 0,
            value: 8,
        };

        let mut board = Board::new(size, &vec![]).unwrap();
        board.toggle_flag(0, 0);

        board.apply(&vec![cell_open]).unwrap();
        assert_eq!(Some(CellState::Open(Opened::Number(8))), board.get(0, 0));
    }
}
