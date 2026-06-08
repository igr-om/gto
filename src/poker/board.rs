use crate::poker::cards::Card;
use crate::poker::deck::Deck;
use rand::Rng;

#[derive(Clone)]
pub struct Board {
    pub cards: Vec<Card>,
}

impl Board {
    pub fn complete_to_river<R: Rng>(&mut self, deck: &mut Deck, rng: &mut R) {
        let missing = 5 - self.cards.len();
        for _ in 0..missing {
            self.cards.push(deck.draw_random(rng));
        }
    }
}
