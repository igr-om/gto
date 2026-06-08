pub mod nlhe;
pub mod plo;
use tracing_subscriber::{fmt, EnvFilter};
use crate::poker::cards::Card;

#[derive(Debug)]
pub enum HoleCards {
    Holdem([Card; 2]),
    Omaha([Card; 4]),
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .init();

    // rest of your main
}
