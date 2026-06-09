use crate::poker::cards::Card;
use crate::poker::range::{Range, WeightedHand};

pub fn parse_card(s: &str) -> Option<Card> {
    if s.len() != 2 {
        return None;
    }

    let rank_char = &s[0..1];
    let suit = s.chars().nth(1)?;

    let rank = match rank_char {
        "A" => 14,
        "K" => 13,
        "Q" => 12,
        "J" => 11,
        "T" => 10,
        n if n.chars().all(|c| c.is_ascii_digit()) => n.parse().ok()?,
        _ => return None,
    };

    Some(Card { rank, suit })
}

pub fn parse_hand(s: &str) -> Vec<Card> {
    s.as_bytes()
        .chunks(2)
        .filter_map(|chunk| std::str::from_utf8(chunk).ok())
        .filter_map(parse_card)
        .collect()
}

pub fn parse_board(s: &str) -> Vec<Card> {
    parse_hand(s)
}

pub fn parse_simple_range(s: &str) -> Range {
    let mut hands = Vec::new();

    for token in s.split(',') {
        let token = token.trim();

        // Handle QQ+
        if token.ends_with('+') && token.len() == 3 {
            let rank_char = &token[0..1];
            let suit_char = &token[1..2];

            let start_rank = match rank_char {
                "A" => 14,
                "K" => 13,
                "Q" => 12,
                "J" => 11,
                "T" => 10,
                _ => continue,
            };

            for r in start_rank..=14 {
                let c1 = Card { rank: r, suit: 'h' };
                let c2 = Card { rank: r, suit: 'd' };
                hands.push(WeightedHand {
                    cards: vec![c1, c2],
                    weight: 1.0,
                });
            }

            continue;
        }

        // Handle AK, QQ, etc.
        if token.len() == 2 {
            let r1 = parse_card(&format!("{}s", &token[0..1]));
            let r2 = parse_card(&format!("{}h", &token[1..2]));

            if let (Some(c1), Some(c2)) = (r1, r2) {
                hands.push(WeightedHand {
                    cards: vec![c1, c2],
                    weight: 1.0,
                });
            }

            continue;
        }
    }

    Range::new(hands)
}
