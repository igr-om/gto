use crossbeam::channel::{unbounded, Receiver, Sender};
use crossbeam::deque::{Injector, Steal, Stealer, Worker};
use rand::Rng;
use std::sync::Arc;
use std::thread;
use tracing::{trace, debug, info, warn};
use crate::poker::{
    board::Board,
    cards::Card,
    deck::Deck,
    range::Range,
};
use crate::variants::{nlhe, plo};

//
// Public API
//

pub enum Task {
    NlheMonteCarlo {
        hand_id: u64,
        iterations: u32,
        hero_hole: [Card; 2],
        board: Board,
        villain_range: Range,
    },
    PloMonteCarlo {
        hand_id: u64,
        iterations: u32,
        hero_hole: [Card; 4],
        board: Board,
        villain_range: Range,
    },
    DataAggregation,
}

pub struct ResultChunk {
    pub hand_id: u64,
    pub wins: u64,
    pub ties: u64,
    pub total: u64,
}

pub struct MonteCarloPool {
    task_injector: Arc<Injector<Task>>,
    result_rx: Receiver<ResultChunk>,
    _handles: Vec<thread::JoinHandle<()>>,
}

impl MonteCarloPool {
    pub fn new(num_workers: usize) -> Self {
        info!("initializing MonteCarloPool with {} workers", num_workers);

        let task_injector = Arc::new(Injector::new());
        let (result_tx, result_rx) = unbounded();

        let mut workers = Vec::new();
        let mut stealers = Vec::new();

        for _ in 0..num_workers {
            let w = Worker::new_fifo();
            stealers.push(w.stealer());
            workers.push(w);
        }

        let global = task_injector.clone();
        let mut handles = Vec::new();

        for (id, local) in workers.into_iter().enumerate() {
            let global = global.clone();
            let result_tx = result_tx.clone();

            let mut others = Vec::new();
            for (j, s) in stealers.iter().enumerate() {
                if j != id {
                    others.push(s.clone());
                }
            }

            let state = WorkerState {
                id,
                local,
                stealers: others,
                global,
                result_tx,
            };

            handles.push(thread::spawn(move || state.run()));
        }

        Self {
            task_injector,
            result_rx,
            _handles: handles,
        }
    }

    pub fn submit_task(&self, task: Task) {
        trace!("submitting task");
        self.task_injector.push(task);
    }

    pub fn results(&self) -> &Receiver<ResultChunk> {
        &self.result_rx
    }
}

//
// Worker internals
//

struct WorkerState {
    id: usize,
    local: Worker<Task>,
    stealers: Vec<Stealer<Task>>,
    global: Arc<Injector<Task>>,
    result_tx: Sender<ResultChunk>,
}

impl WorkerState {
    fn run(mut self) {
        info!("worker {} starting", self.id);

        let mut rng = rand::thread_rng();

        loop {
            // 1) local queue
            if let Some(task) = self.local.pop() {
                trace!("worker {} executing local task", self.id);
                self.handle_task(task, &mut rng);
                continue;
            }

            // 2) global injector
            if let Some(task) = self.global.steal().success() {
                trace!("worker {} executing global task", self.id);
                self.handle_task(task, &mut rng);
                continue;
            }

            // 3) steal from other workers
            let mut stole = false;
            for stealer in &self.stealers {
                match stealer.steal() {
                    Steal::Success(task) => {
                        trace!("worker {} stole task", self.id);
                        self.handle_task(task, &mut rng);
                        stole = true;
                        break;
                    }
                    Steal::Retry => trace!("worker {} retrying steal", self.id),
                    Steal::Empty => {}
                }
            }

            if !stole {
                info!("worker {} shutting down (no more work)", self.id);
                break;
            }
        }
    }

    fn handle_task(&self, task: Task, rng: &mut impl Rng) {
        match task {
            Task::NlheMonteCarlo {
                hand_id,
                iterations,
                hero_hole,
                board,
                villain_range,
            } => {
                debug!(
                    "worker {} running NLHE job {} ({} iterations)",
                    self.id, hand_id, iterations
                );

                let (wins, ties, total) =
                    run_nlhe_monte_carlo(rng, hero_hole, &board, iterations, &villain_range);

                trace!(
                    "worker {} NLHE job {} chunk: wins={}, ties={}, total={}",
                    self.id,
                    hand_id,
                    wins,
                    ties,
                    total
                );

                let _ = self.result_tx.send(ResultChunk {
                    hand_id,
                    wins,
                    ties,
                    total,
                });
            }

            Task::PloMonteCarlo {
                hand_id,
                iterations,
                hero_hole,
                board,
                villain_range,
            } => {
                debug!(
                    "worker {} running PLO job {} ({} iterations)",
                    self.id, hand_id, iterations
                );

                let (wins, ties, total) =
                    run_plo_monte_carlo(rng, hero_hole, &board, iterations, &villain_range);

                trace!(
                    "worker {} PLO job {} chunk: wins={}, ties={}, total={}",
                    self.id,
                    hand_id,
                    wins,
                    ties,
                    total
                );

                let _ = self.result_tx.send(ResultChunk {
                    hand_id,
                    wins,
                    ties,
                    total,
                });
            }

            Task::DataAggregation => {
                warn!("worker {} received DataAggregation task (not implemented)", self.id);
            }
        }
    }
}

//
// Game-specific Monte Carlo loops
//

fn run_nlhe_monte_carlo<R: Rng>(
    rng: &mut R,
    hero_hole: [Card; 2],
    board: &Board,
    iterations: u32,
    villain_range: &Range,
) -> (u64, u64, u64) {
    let mut wins = 0;
    let mut ties = 0;
    let mut total = 0;

    let mut deck = Deck::new();

    for i in 0..iterations {
        deck.reset();
        deck.remove_many(&hero_hole);
        deck.remove_many(&board.cards);

        // Dead cards for blocker-aware sampling
        let mut dead = hero_hole.to_vec();
        dead.extend_from_slice(&board.cards);

        let villain = match villain_range.sample_hand(rng, &dead) {
            Some(v) => [v[0], v[1]],
            None => continue,
        };

        let mut full_board = board.clone();
        full_board.complete_to_river(&mut deck, rng);

        let hero_rank = nlhe::evaluate_hand(&hero_hole, &full_board);
        let villain_rank = nlhe::evaluate_hand(&villain, &full_board);

        if hero_rank > villain_rank {
            wins += 1;
        } else if hero_rank == villain_rank {
            ties += 1;
        }

        total += 1;

        trace!(
            "NLHE iter {}: hero={:?} villain={:?} result={:?}",
            i,
            hero_hole,
            villain,
            (hero_rank, villain_rank)
        );
    }

    (wins, ties, total)
}

fn run_plo_monte_carlo<R: Rng>(
    rng: &mut R,
    hero_hole: [Card; 4],
    board: &Board,
    iterations: u32,
    villain_range: &Range,
) -> (u64, u64, u64) {
    let mut wins = 0;
    let mut ties = 0;
    let mut total = 0;

    let mut deck = Deck::new();

    for i in 0..iterations {
        deck.reset();
        deck.remove_many(&hero_hole);
        deck.remove_many(&board.cards);

        let mut dead = hero_hole.to_vec();
        dead.extend_from_slice(&board.cards);

        let villain = match villain_range.sample_hand(rng, &dead) {
            Some(v) => [v[0], v[1], v[2], v[3]],
            None => continue,
        };

        let mut full_board = board.clone();
        full_board.complete_to_river(&mut deck, rng);

        let hero_rank = plo::evaluate_hand(&hero_hole, &full_board);
        let villain_rank = plo::evaluate_hand(&villain, &full_board);

        if hero_rank > villain_rank {
            wins += 1;
        } else if hero_rank == villain_rank {
            ties += 1;
        }

        total += 1;

        trace!(
            "PLO iter {}: hero={:?} villain={:?} result={:?}",
            i,
            hero_hole,
            villain,
            (hero_rank, villain_rank)
        );
    }

    (wins, ties, total)
}
