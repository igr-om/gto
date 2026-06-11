use std::collections::HashMap;
use std::str::FromStr;

use crate::poker::cards::Card;

/* ============================
   POSITION + ACTION ENUMS
   ============================ */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Position {
    UTG,
    HJ,
    CO,
    BTN,
    SB,
    BB,
}

impl FromStr for Position {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "UTG" => Ok(Position::UTG),
            "HJ"  => Ok(Position::HJ),
            "CO"  => Ok(Position::CO),
            "BTN" => Ok(Position::BTN),
            "SB"  => Ok(Position::SB),
            "BB"  => Ok(Position::BB),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Open,
    Call,
    ThreeBet,
    FourBet,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "OPEN" => Ok(Action::Open),
            "CALL" => Ok(Action::Call),
            "3BET" | "THREEBET" => Ok(Action::ThreeBet),
            "4BET" | "FOURBET"  => Ok(Action::FourBet),
            _ => Err(()),
        }
    }
}

/* ============================
   COMBO ENTRY (freq + EV)
   ============================ */

#[derive(Clone, Debug)]
pub struct ComboEntry {
    pub freq: f64,
    pub ev: f64,
}

/* ============================
   COMBO RANGE
   ============================ */

#[derive(Clone, Debug)]
pub struct ComboRange {
    pub hands: HashMap<String, ComboEntry>,
}

impl ComboRange {
    pub fn new() -> Self {
        Self {
            hands: HashMap::new(),
        }
    }

    pub fn insert(&mut self, hand: String, freq: f64, ev: f64) {
        self.hands.insert(hand, ComboEntry { freq, ev });
    }

    pub fn frequency_for_hand(&self, hand: &str) -> Option<f64> {
        self.hands.get(hand).map(|e| e.freq)
    }

    pub fn ev_for_hand(&self, hand: &str) -> Option<f64> {
        self.hands.get(hand).map(|e| e.ev)
    }
}

/* ============================
   RANGE TABLE (POS × ACTION)
   ============================ */

#[derive(Clone, Debug)]
pub struct RangeTable {
    pub table: HashMap<(Position, Action), ComboRange>,
}

impl RangeTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub fn add_entry(
        &mut self,
        pos: Position,
        act: Action,
        hand: String,
        freq: f64,
        ev: f64,
    ) {
        self.table
            .entry((pos, act))
            .or_insert_with(ComboRange::new)
            .insert(hand, freq, ev);
    }

    pub fn to_combo_range(
        &self,
        pos: Position,
        act: Action,
    ) -> Option<ComboRange> {
        self.table.get(&(pos, act)).cloned()
    }
}

/* ============================
   HAND EXPANSION (AKs → combos)
   ============================ */

pub fn expand_hand_to_combos(code: &str) -> Vec<[Card; 2]> {
    let chars: Vec<char> = code.chars().collect();
    if chars.len() < 2 {
        return vec![];
    }

    let r1 = rank_to_value(chars[0]);
    let r2 = rank_to_value(chars[1]);

    let suited = chars.len() == 3 && chars[2] == 's';
    let offsuit = chars.len() == 3 && chars[2] == 'o';
    let pair = r1 == r2;

    let suits = ['h', 'd', 'c', 's'];
    let mut combos = vec![];

    if pair {
        for i in 0..4 {
            for j in (i + 1)..4 {
                combos.push([
                    Card { rank: r1, suit: suits[i] },
                    Card { rank: r2, suit: suits[j] },
                ]);
            }
        }
        return combos;
    }

    for &s1 in &suits {
        for &s2 in &suits {
            if s1 == s2 && offsuit {
                continue;
            }
            if s1 != s2 && suited {
                continue;
            }
            combos.push([
                Card { rank: r1, suit: s1 },
                Card { rank: r2, suit: s2 },
            ]);
        }
    }

    combos
}

/* ============================
   RANK PARSER
   ============================ */

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
        _ => panic!("Invalid rank"),
    }
}
