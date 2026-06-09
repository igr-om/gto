use crate::poker::{
    cards::Card,
    board::Board,
    ranking::{HandRank, evaluate_7},
};
use crate::poker::deck::Deck;
use rand::Rng;

pub fn sample_villain_hand<R: Rng>(rng: &mut R, deck: &mut Deck) -> [Card; 2] {
    let c1 = deck.draw_random(rng);
    let c2 = deck.draw_random(rng);
    [c1, c2]
}


/// Evaluate hero’s NLHE hand (2 hole + board) as a 7-card hand.
pub fn evaluate_hand(hole: &[Card; 2], board: &Board) -> HandRank {
    let mut all = [hole[0], hole[1],
        Card { rank: 0, suit: 'x' },
        Card { rank: 0, suit: 'x' },
        Card { rank: 0, suit: 'x' },
        Card { rank: 0, suit: 'x' },
        Card { rank: 0, suit: 'x' },
    ];

    for (i, c) in board.cards.iter().enumerate() {
        all[2 + i] = *c;
    }

    evaluate_7(&all)
}