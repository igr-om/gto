use crate::poker::gto_range::{Position, Action, RangeTable, expand_hand_to_combos};
use rand::Rng;
use serde::Serialize;
use crate::poker::cards::Card;

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

impl Drill {
    pub fn generate<R: Rng>(
        rng: &mut R,
        gto: &RangeTable,
        position: Position,
        action: Action,
    ) -> Option<Self> {
        let combo_range = gto.to_combo_range(position, action)?;

        let dead: &[Card] = &[];
        let hand = combo_range.sample_hand(rng, dead)?;

        Some(Self {
            position,
            action,
            hand,
        })
    }
}
