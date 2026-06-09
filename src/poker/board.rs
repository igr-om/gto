use rand::Rng;
use crate::poker::cards::Card;
use crate::poker::deck::Deck;

#[derive(Clone, Debug)]
pub struct Board {
    pub cards: Vec<Card>,
}

impl Board {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    pub fn from_cards(cards: Vec<Card>) -> Self {
        Self { cards }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn complete_to_river<R: Rng>(&mut self, deck: &mut Deck, rng: &mut R) {
        while self.cards.len() < 5 {
            let c = deck.draw_random(rng);
            self.cards.push(c);
        }
    }
}
