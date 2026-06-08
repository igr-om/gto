use crate::poker::cards::Card;
use rand::Rng;
use crate::poker::{
    board::Board,
    cards::Card,
    deck::Deck,
    range::Range,
};

#[derive(Clone)]
pub struct WeightedHand {
    pub cards: Vec<Card>, // 2 for NLHE, 4 for PLO
    pub weight: f64,
}

#[derive(Clone)]
pub struct Range {
    pub hands: Vec<WeightedHand>,
    pub total_weight: f64,
}

impl Range {
    pub fn sample_hand<R: Rng>(
        &self,
        rng: &mut R,
        dead: &[Card],
    ) -> Option<Vec<Card>> {
        let mut target = rng.gen::<f64>() * self.total_weight;

        for h in &self.hands {
            // Blocker check
            if h.cards.iter().any(|c| dead.contains(c)) {
                continue;
            }

            if target <= h.weight {
                return Some(h.cards.clone());
            }

            target -= h.weight;
        }

        None
    }
}