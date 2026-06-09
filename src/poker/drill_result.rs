use serde::Serialize;
use crate::poker::cards::Card;
use crate::poker::gto_range::{Position, Action};

#[derive(Debug, Clone, Serialize)]
pub struct Drill {
    pub position: Position,
    pub action: Action,
    pub hand: Vec<Card>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DrillResult {
    pub position: Position,
    pub action: Action,
    pub hand: Vec<Card>,
    pub user_action: Action,
    pub correct: bool,
}