use crate::cards::Card;

#[derive(Debug)]
pub enum HoleCards {
    Holdem([Card; 2]),
    Omaha([Card; 4]),
}
