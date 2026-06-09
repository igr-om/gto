use crate::poker::{cards::Card, board::Board};
use crate::poker::ranking::{HandRank, evaluate_5};

pub fn evaluate_hand(hole: &[Card; 4], board: &Board) -> HandRank {
    debug_assert!(board.cards.len() == 5, "PLO evaluation requires full 5-card board");

    let b = &board.cards;

    let mut best = HandRank(0);

    // 6 hole-pairs: (0,1)(0,2)(0,3)(1,2)(1,3)(2,3)
    let hole_pairs = [
        (0usize, 1usize),
        (0, 2),
        (0, 3),
        (1, 2),
        (1, 3),
        (2, 3),
    ];

    // 10 board triplets: all 3-combos of 0..5
    let board_trips = [
        (0usize,1usize,2usize),
        (0,1,3),
        (0,1,4),
        (0,2,3),
        (0,2,4),
        (0,3,4),
        (1,2,3),
        (1,2,4),
        (1,3,4),
        (2,3,4),
    ];

    for &(hi, hj) in &hole_pairs {
        let h0 = hole[hi];
        let h1 = hole[hj];

        for &(bi, bj, bk) in &board_trips {
            let five = [
                h0,
                h1,
                b[bi],
                b[bj],
                b[bk],
            ];

            let rank = evaluate_5(&five);
            if rank > best {
                best = rank;
            }
        }
    }

    best
}