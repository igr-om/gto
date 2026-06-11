use rand::seq::SliceRandom;
use rand::Rng;
use crate::poker::cards::Card;

pub fn combinations<T: Clone>(items: &[T], k: usize) -> Vec<Vec<T>> {
    Vec::new()
}
pub fn hand_label(r: usize, c: usize) -> String {
    // Ranks from strongest to weakest
    const RANKS: [&str; 13] = [
        "A","K","Q","J","T","9","8","7","6","5","4","3","2"
    ];

    let rank1 = RANKS[r];
    let rank2 = RANKS[c];

    if r < c {
        format!("{}{}s", rank1, rank2) // suited
    } else if r > c {
        format!("{}{}o", rank1, rank2) // offsuit
    } else {
        format!("{}{}", rank1, rank2)  // pair
    }
}

pub fn random_hand(dead: &[Card]) -> [Card; 2] {
    let ranks = [14,13,12,11,10,9,8,7,6,5,4,3,2];
    let suits = ['h','d','c','s'];

    let mut deck = Vec::new();
    for &r in &ranks {
        for &s in &suits {
            let c = Card { rank: r, suit: s };
            if !dead.contains(&c) {
                deck.push(c);
            }
        }
    }

    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);

    [deck[0], deck[1]]
}

