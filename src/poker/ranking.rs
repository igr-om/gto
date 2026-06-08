use crate::poker::cards::Card;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HandRank(pub u64);

impl HandRank {
    pub fn category(&self) -> u8 {
        (self.0 >> 60) as u8
    }
}

pub fn evaluate_5(cards: &[Card; 5]) -> HandRank {
    // Rank mask: bit r means rank r is present
    let mut rank_mask: u32 = 0;
    let mut suit_counts = [0u8; 4]; // h,d,c,s
    let mut rank_counts = [0u8; 15]; // ranks 2..14

    for c in cards {
        rank_mask |= 1 << c.rank;
        rank_counts[c.rank as usize] += 1;

        let suit_idx = match c.suit {
            'h' => 0,
            'd' => 1,
            'c' => 2,
            's' => 3,
            _ => unreachable!(),
        };
        suit_counts[suit_idx] += 1;
    }

    // Detect flush
    let is_flush = suit_counts.iter().any(|&c| c == 5);

    // Detect straight via bitmask patterns
    let mut is_straight = false;
    let mut top_straight = 0;

    // Normal straight: 5 consecutive bits set
    for high in (5..=14).rev() {
        let window = (0b11111u32) << (high - 4);
        if rank_mask & window == window {
            is_straight = true;
            top_straight = high;
            break;
        }
    }

    // Wheel straight (A-2-3-4-5)
    if !is_straight {
        let wheel = (1 << 14) | (1 << 5) | (1 << 4) | (1 << 3) | (1 << 2);
        if rank_mask & wheel == wheel {
            is_straight = true;
            top_straight = 5;
        }
    }

    // Build groups: (count, rank)
    let mut groups = Vec::new();
    for r in (2..=14).rev() {
        let c = rank_counts[r];
        if c > 0 {
            groups.push((c, r as u8));
        }
    }

    groups.sort_unstable_by(|a, b| b.cmp(a));

    // Category detection
    let category: u8;
    let mut kickers = Vec::new();

    if is_straight && is_flush {
        category = 8;
        kickers.push(top_straight);
    } else if groups[0].0 == 4 {
        category = 7;
        kickers.push(groups[0].1);
        kickers.push(groups[1].1);
    } else if groups[0].0 == 3 && groups[1].0 == 2 {
        category = 6;
        kickers.push(groups[0].1);
        kickers.push(groups[1].1);
    } else if is_flush {
        category = 5;
        for r in (2..=14).rev() {
            if rank_counts[r] > 0 {
                kickers.push(r as u8);
            }
        }
    } else if is_straight {
        category = 4;
        kickers.push(top_straight);
    } else if groups[0].0 == 3 {
        category = 3;
        kickers.push(groups[0].1);
        for g in groups.iter().skip(1) {
            kickers.push(g.1);
        }
    } else if groups[0].0 == 2 && groups[1].0 == 2 {
        category = 2;
        kickers.push(groups[0].1);
        kickers.push(groups[1].1);
        kickers.push(groups[2].1);
    } else if groups[0].0 == 2 {
        category = 1;
        kickers.push(groups[0].1);
        for g in groups.iter().skip(1) {
            kickers.push(g.1);
        }
    } else {
        category = 0;
        for r in (2..=14).rev() {
            if rank_counts[r] > 0 {
                kickers.push(r as u8);
            }
        }
    }

    // Pack category + kickers into u64
    let mut value = (category as u64) << 60;
    for k in kickers {
        value = (value << 4) | (k as u64 & 0xF);
    }

    HandRank(value)
}

pub fn evaluate_7(cards: &[Card; 7]) -> HandRank {
    // 21 combinations of 7 choose 5
    const COMBOS: [[usize; 5]; 21] = [
        [0,1,2,3,4],[0,1,2,3,5],[0,1,2,3,6],
        [0,1,2,4,5],[0,1,2,4,6],[0,1,2,5,6],
        [0,1,3,4,5],[0,1,3,4,6],[0,1,3,5,6],
        [0,1,4,5,6],[0,2,3,4,5],[0,2,3,4,6],
        [0,2,3,5,6],[0,2,4,5,6],[0,3,4,5,6],
        [1,2,3,4,5],[1,2,3,4,6],[1,2,3,5,6],
        [1,2,4,5,6],[1,3,4,5,6],[2,3,4,5,6],
    ];

    let mut best = HandRank(0);

    for idx in COMBOS {
        let five = [
            cards[idx[0]],
            cards[idx[1]],
            cards[idx[2]],
            cards[idx[3]],
            cards[idx[4]],
        ];
        let rank = evaluate_5(&five);
        if rank > best {
            best = rank;
        }
    }

    best
}
