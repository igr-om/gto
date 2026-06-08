mod poker;
mod variants;
mod compute;

use compute::monte_carlo::{MonteCarloPool, Task};
use poker::cards::Card;
use poker::range::{Range, WeightedHand};
use rand::thread_rng;

fn main() {
    // Example: build a tiny range
    let hands = vec![
        WeightedHand {
            cards: vec![
                Card { rank: 14, suit: 'h' },
                Card { rank: 14, suit: 'd' },
            ],
            weight: 1.0,
        },
        WeightedHand {
            cards: vec![
                Card { rank: 13, suit: 's' },
                Card { rank: 13, suit: 'c' },
            ],
            weight: 1.0,
        },
    ];

    let range = Range::new(hands);

    let mut rng = thread_rng();
    let dead: &[Card] = &[];

    if let Some(hand) = range.sample_hand(&mut rng, dead) {
        println!("Sampled hand: {:?}", hand);
    } else {
        println!("No valid hand found");
    }
}