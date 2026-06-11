use std::collections::HashMap;
use crate::poker::gto_range::{Position, Action};

#[derive(Debug, Clone)]
pub struct UserStats {
    pub stats: HashMap<(Position, Action), (u32, u32)>, // (correct, total)
}

impl UserStats {
    pub fn new() -> Self {
        Self { stats: HashMap::new() }
    }

    pub fn record(&mut self, pos: Position, action: Action, correct: bool) {
        let entry = self.stats.entry((pos, action)).or_insert((0, 0));
        if correct {
            entry.0 += 1;
        }
        entry.1 += 1;
    }

    pub fn get(&self, pos: Position, action: Action) -> (u32, u32) {
        self.stats.get(&(pos, action)).cloned().unwrap_or((0, 0))
    }
}
