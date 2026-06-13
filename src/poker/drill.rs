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

        // Select a random hand from the range
        let hands: Vec<_> = combo_range.hands.keys().collect();
        if hands.is_empty() {
            return None;
        }
        
        let hand_code = hands[rng.gen_range(0..hands.len())];
        let expanded = expand_hand_to_combos(hand_code);
        
        let hand = if !expanded.is_empty() {
            let selected = &expanded[rng.gen_range(0..expanded.len())];
            vec![selected[0], selected[1]]
        } else {
            vec![]
        };

        Some(Self {
            position,
            action,
            hand,
        })
    }
}
