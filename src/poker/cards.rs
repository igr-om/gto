use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Card {
    pub rank: u8,
    pub suit: char,
}

pub fn parse_hand(s: &str) -> Vec<Card> {
    s.as_bytes()
        .chunks(2)
        .filter_map(|c| std::str::from_utf8(c).ok())
        .filter_map(parse_card)
        .collect()
}
pub fn parse_card(s: &str) -> Option<Card> {
    if s.len() != 2 { return None; }

    let rank = match &s[0..1] {
        "A" => 14,
        "K" => 13,
        "Q" => 12,
        "J" => 11,
        "T" => 10,
        n if n.chars().all(|c| c.is_ascii_digit()) => n.parse().ok()?,
        _ => return None,
    };

    let suit = s.chars().nth(1)?;

    Some(Card { rank, suit })
}


