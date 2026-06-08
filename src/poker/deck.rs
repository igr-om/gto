use crate::poker::cards::Card;
use rand::Rng;

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);
        for &suit in &['h','d','c','s'] {
            for rank in 2..=14 {
                cards.push(Card { rank, suit });
            }
        }
        Self { cards }
    }

    pub fn remove(&mut self, card: Card) {
        self.cards.retain(|c| !(c.rank == card.rank && c.suit == card.suit));
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

    pub fn reset(&mut self) {
        self.cards.clear();
        self.cards.reserve(52);

        for rank in 2..=14 {
            self.cards.push(Card { rank, suit: 'h' });
            self.cards.push(Card { rank, suit: 'd' });
            self.cards.push(Card { rank, suit: 'c' });
            self.cards.push(Card { rank, suit: 's' });
        }
    }
}