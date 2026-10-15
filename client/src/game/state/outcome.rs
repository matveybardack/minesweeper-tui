/// Reason of game ending
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Won,
    LostByOpponent,
    LostBySurrender,
    LostByTimeout,
    LostByMine,
}
