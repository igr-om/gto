use crate::poker::cards::Card;
use rand::Rng;

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn test_range_sampling() {
        let hands = vec![
            WeightedHand {
                cards: vec![Card { rank: 14, suit: 'h' }, Card { rank: 14, suit: 'd' }],
                weight: 1.0,
            },
            WeightedHand {
                cards: vec![Card { rank: 13, suit: 's' }, Card { rank: 13, suit: 'c' }],
                weight: 1.0,
            },
        ];

        let range = Range::new(hands);
        let mut rng = thread_rng();
        let dead: &[Card] = &[];

        assert!(range.sample_hand(&mut rng, dead).is_some());
    }
}

#[derive(Clone)]
pub struct WeightedHand {
    pub cards: Vec<Card>, // 2 for NLHE, 4 for PLO
    pub weight: f64,
}

#[derive(Clone)]
pub struct Range {
    pub hands: Vec<WeightedHand>,
    pub total_weight: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum HandType {
    Suited,
    Offsuit,
    Pair,
}

impl Range {
    pub fn new(hands: Vec<WeightedHand>) -> Self {
        let total_weight = hands.iter().map(|h| h.weight).sum();
        Self { hands, total_weight }
    }

    pub fn sample_hand<R: Rng>(
        &self,
        rng: &mut R,
        dead: &[Card],
    ) -> Option<Vec<Card>> {
        
        let mut target = (rng.r#gen::<f64>()) * self.total_weight;

        for h in &self.hands {
            if h.cards.iter().any(|c| dead.contains(c)) {
                continue;
            }

            if target <= h.weight {
                return Some(h.cards.clone());
            }

            target -= h.weight;
        }

        None
    }
}

pub fn parse_hand_code(code: &str) -> (u8, u8, HandType) {
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
        panic!("Invalid hand code");
    };

    (r1, r2, hand_type)
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
        _ => panic!("Invalid rank"),
    }
}