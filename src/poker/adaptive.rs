use crate::poker::gto_range::{Position, Action};
use crate::poker::user_stats::UserStats;
use rand::Rng;

pub struct AdaptiveEngine;

impl AdaptiveEngine {
    pub fn next_action<R: Rng>(
        rng: &mut R,
        stats: &UserStats,
    ) -> (Position, Action) {
        // Find weakest area
        let mut weakest: Option<((Position, Action), f32)> = None;

        for ((pos, act), (correct, total)) in &stats.stats {
            let acc = *correct as f32 / *total as f32;

            if weakest.is_none() || acc < weakest.unwrap().1 {
                weakest = Some(((*pos, *act), acc));
            }
        }

        // If no data yet → random
        if let Some(((pos, act), _)) = weakest {
            (pos, act)
        } else {
            // fallback: random drill
            let positions = [Position::UTG, Position::MP, Position::CO, Position::BTN, Position::SB, Position::BB];
            let actions = [Action::Open, Action::Call, Action::ThreeBet, Action::FourBet, Action::Defend];

            let pos = positions[rng.gen_range(0..positions.len())];
            let act = actions[rng.gen_range(0..actions.len())];

            (pos, act)
        }
    }
}
