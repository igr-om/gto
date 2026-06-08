use crossbeam::channel::{unbounded, Sender, Receiver};
use crossbeam::deque::{Injector, Steal, Stealer, Worker};
use rand::Rng;
use std::sync::Arc;
use std::thread;

#[derive(Debug, Clone)]
pub enum Task {
    MonteCarloSample {
        hand_id: u64,
        iterations: u32,
        // add fields: hero hand, villain range, board, variant, etc.
    },
    DataAggregation,
}

#[derive(Debug, Clone)]
pub struct ResultChunk {
    pub hand_id: u64,
    pub wins: u64,
    pub ties: u64,
    pub total: u64,
}
pub mod monte_carlo;
