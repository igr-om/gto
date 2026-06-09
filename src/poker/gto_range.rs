use std::collections::HashMap;
use std::str::FromStr;
use serde::{Serialize, Deserialize};

use crate::poker::cards::Card;
use crate::poker::range::{Range, WeightedHand};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Position {
    UTG, MP, CO, BTN, SB, BB,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Action {
    Open,
    Call,
    ThreeBet,
    FourBet,
    Defend,
}

#[derive(Debug, Clone)]
pub struct RangeEntry {
    pub hand: String,   // "AKs", "QJo", "77"
    pub weight: f32,
}

pub struct RangeTable {
    pub table: HashMap<(Position, Action), Vec<RangeEntry>>,
}

#[derive(Debug, Clone, Copy)]
pub enum HandType {
    Suited,
    Offsuit,
    Pair,
}

impl FromStr for Position {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UTG" => Ok(Position::UTG),
            "MP"  => Ok(Position::MP),
            "CO"  => Ok(Position::CO),
            "BTN" => Ok(Position::BTN),
            "SB"  => Ok(Position::SB),
            "BB"  => Ok(Position::BB),
            _ => Err(()),
        }
    }
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Open"     => Ok(Action::Open),
            "Call"     => Ok(Action::Call),
            "ThreeBet" => Ok(Action::ThreeBet),
            "FourBet"  => Ok(Action::FourBet),
            "Defend"   => Ok(Action::Defend),
            _ => Err(()),
        }
    }
}

impl std::str::FromStr for Action {
    type Err = ();

use std::str::FromStr;

impl FromStr for Position {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UTG" => Ok(Position::UTG),
            "MP"  => Ok(Position::MP),
            "CO"  => Ok(Position::CO),
            "BTN" => Ok(Position::BTN),
            "SB"  => Ok(Position::SB),
            "BB"  => Ok(Position::BB),
            _ => Err(()),
        }
    }
}

impl FromStr for Action {
    type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "Open" => Ok(Action::Open),
                "Call" => Ok(Action::Call),
                "ThreeBet" => Ok(Action::ThreeBet),
                "FourBet" => Ok(Action::FourBet),
                "Defend" => Ok(Action::Defend),
                _ => Err(()),
            }
        }
    }
}

impl RangeTable {
    pub fn new() -> Self {
        Self { table: HashMap::new() }
    }

    pub fn add_entry(
        &mut self,
        pos: Position,
        action: Action,
        hand: impl Into<String>,
        weight: f32,
    ) {
        self.table
            .entry((pos, action))
            .or_insert_with(Vec::new)
            .push(RangeEntry {
                hand: hand.into(),
                weight,
            });
    }

    pub fn to_combo_range(
        &self,
        pos: Position,
        action: Action,
    ) -> Option<Range> {
        let entries = self.table.get(&(pos, action))?;

        let mut hands = Vec::new();

        for entry in entries {
            let combos = expand_hand_to_combos(&entry.hand);

            for combo in combos {
                hands.push(WeightedHand {
                    cards: combo,
                    weight: entry.weight as f64,
                });
            }
        }

        Some(Range::new(hands))
    }
}

fn rank_to_value(c: char) -> u8 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => panic!("Invalid rank {}", c),
    }
}

fn parse_hand_code(code: &str) -> (u8, u8, HandType) {
    let chars: Vec<char> = code.chars().collect();

    let r1 = rank_to_value(chars[0]);
    let r2 = rank_to_value(chars[1]);

    let hand_type = if chars.len() == 3 {
        match chars[2] {
            's' => HandType::Suited,
            'o' => HandType::Offsuit,
            _ => panic!("Invalid hand type"),
        }
    } else if r1 == r2 {
        HandType::Pair
    } else {
        panic!("Invalid hand code {}", code);
    };

    (r1, r2, hand_type)
}

pub fn expand_hand_to_combos(code: &str) -> Vec<Vec<Card>> {
    let (r1, r2, hand_type) = parse_hand_code(code);

    let suits = ['h', 'd', 'c', 's'];
    let mut combos = Vec::new();

    match hand_type {
        HandType::Pair => {
            for i in 0..4 {
                for j in (i + 1)..4 {
                    combos.push(vec![
                        Card { rank: r1, suit: suits[i] },
                        Card { rank: r1, suit: suits[j] },
                    ]);
                }
            }
        }

        HandType::Suited => {
            for s in suits {
                combos.push(vec![
                    Card { rank: r1, suit: s },
                    Card { rank: r2, suit: s },
                ]);
            }
        }

        HandType::Offsuit => {
            for s1 in suits {
                for s2 in suits {
                    if s1 != s2 {
                        combos.push(vec![
                            Card { rank: r1, suit: s1 },
                            Card { rank: r2, suit: s2 },
                        ]);
                    }
                }
            }
        }
    }

    combos
}
