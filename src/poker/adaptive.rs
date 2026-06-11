use rand::Rng;

use crate::poker::gto_range::{Position, Action};
use crate::poker::user_stats::UserStats;

pub struct AdaptiveEngine;

impl AdaptiveEngine {
    pub fn next_action<R: Rng>(
        rng: &mut R,
        stats: &UserStats,
    ) -> (Position, Action) {
        let positions = [
            Position::UTG,
            Position::MP,
            Position::HJ,
            Position::CO,
            Position::BTN,
            Position::SB,
            Position::BB,
        ];

        let actions = [
            Action::Open,
            Action::Call,
            Action::ThreeBet,
            Action::FourBet,
            Action::Defend,
        ];

        let mut weighted: Vec<((Position, Action), f64)> = Vec::new();

        for &pos in &positions {
            for &act in &actions {
                let (correct, total) = stats.get(pos, act);

                let accuracy = if total == 0 {
                    0.5
                } else {
                    correct as f64 / total as f64
                };

                let weight = (1.0 - accuracy) + 0.1;

                weighted.push(((pos, act), weight));
            }
        }

        let sum: f64 = weighted.iter().map(|(_, w)| *w).sum();
        let mut roll = rng.gen::<f64>() * sum;

        for &((pos, act), w) in &weighted {
            if roll < w {
                return (pos, act);
            }
            roll -= w;
        }

        weighted
            .first()
            .map(|((pos, act), _)| (*pos, *act))
            .unwrap_or((Position::UTG, Action::Open))
    }
}
