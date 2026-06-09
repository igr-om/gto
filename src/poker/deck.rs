use rand::Rng;
use crate::poker::cards::Card;

#[derive(Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);

        for rank in 2..=14 {
            for suit in ['h', 'd', 'c', 's'] {
                cards.push(Card { rank, suit });
            }
        }

        Self { cards }
    }

    pub fn reset(&mut self) {
        self.cards.clear();
        for rank in 2..=14 {
            for suit in ['h', 'd', 'c', 's'] {
                self.cards.push(Card { rank, suit });
            }
        }
    }

    pub fn remove(&mut self, card: Card) {
        if let Some(pos) = self.cards.iter().position(|c| *c == card) {
            self.cards.swap_remove(pos);
        }
    }

    pub fn remove_many(&mut self, cards: &[Card]) {
        for c in cards {
            self.remove(*c);
        }
    }

    pub fn draw_random<R: Rng>(&mut self, rng: &mut R) -> Card {
        let idx = rng.gen_range(0..self.cards.len());
        self.cards.swap_remove(idx)
    }
}
