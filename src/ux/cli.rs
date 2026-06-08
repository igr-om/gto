use crate::poker::cards::Card;
use crate::variants::{HoleCards};
use crate::poker::board::Board;
use crate::poker::equity::calculate_equity;

pub fn run() {
    let h = HoleCards::Holdem([
        Card { rank: 14, suit: 'h' },
        Card { rank: 13, suit: 'h' },
    ]);

    let board = Board {
        cards: vec![
            Card { rank: 2, suit: 'c' },
            Card { rank: 7, suit: 'd' },
            Card { rank: 10, suit: 'h' },
        ],
    };

    let eq = calculate_equity(&h, &board);
    println!("Equity: {:.1}%", eq * 100.0);
}
