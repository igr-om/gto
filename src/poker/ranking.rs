use crate::poker::cards::Card;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HandRank(pub u32);

//
// 5-card evaluator
//

pub fn evaluate_5(cards: &[Card; 5]) -> HandRank {
    let mut ranks = [0u8; 5];
    let mut suits = [0u8; 5];

    for i in 0..5 {
        ranks[i] = cards[i].rank;
        suits[i] = cards[i].suit as u8;
    }

    ranks.sort_unstable_by(|a, b| b.cmp(a));

    let is_flush = suits.iter().all(|&s| s == suits[0]);

    // --- Bitmask straight detection ---
    let mut mask: u32 = 0;
    for &r in &ranks {
        mask |= 1 << r;
    }

    let wheel = (1 << 14) | (1 << 5) | (1 << 4) | (1 << 3) | (1 << 2);

    let is_straight = if mask & wheel == wheel {
        true
    } else {
        let mut found = false;
        for high in (5..=14).rev() {
            let run = (1 << high)
                | (1 << (high - 1))
                | (1 << (high - 2))
                | (1 << (high - 3))
                | (1 << (high - 4));

            if mask & run == run {
                found = true;
                break;
            }
        }
        found
    };
    // ----------------------------------

    let mut counts = [0u8; 15];
    for &r in &ranks {
        counts[r as usize] += 1;
    }

    let mut pairs = 0;
    let mut trips = 0;
    let mut quads = 0;

    for &c in &counts {
        match c {
            2 => pairs += 1,
            3 => trips += 1,
            4 => quads += 1,
            _ => {}
        }
    }

    let category = if is_straight && is_flush {
        8
    } else if quads == 1 {
        7
    } else if trips == 1 && pairs == 1 {
        6
    } else if is_flush {
        5
    } else if is_straight {
        4
    } else if trips == 1 {
        3
    } else if pairs == 2 {
        2
    } else if pairs == 1 {
        1
    } else {
        0
    };

    let mut kicker = 0u32;
    for &r in &ranks {
        kicker = kicker * 16 + r as u32;
    }

    HandRank((category << 20) | kicker)
}

// 7-card evaluator (21 combos)
pub fn evaluate_7(cards: &[Card; 7]) -> HandRank {
    let mut best = HandRank(0);

    const COMBOS: [(usize, usize, usize, usize, usize); 21] = [
        (0,1,2,3,4),(0,1,2,3,5),(0,1,2,3,6),
        (0,1,2,4,5),(0,1,2,4,6),(0,1,2,5,6),
        (0,1,3,4,5),(0,1,3,4,6),(0,1,3,5,6),
        (0,1,4,5,6),(0,2,3,4,5),(0,2,3,4,6),
        (0,2,3,5,6),(0,2,4,5,6),(0,3,4,5,6),
        (1,2,3,4,5),(1,2,3,4,6),(1,2,3,5,6),
        (1,2,4,5,6),(1,3,4,5,6),(2,3,4,5,6),
    ];

    for &(a,b,c,d,e) in &COMBOS {
        let five = [
            cards[a],
            cards[b],
            cards[c],
            cards[d],
            cards[e],
        ];

        let rank = evaluate_5(&five);
        if rank > best {
            best = rank;
        }
    }

    best
}
