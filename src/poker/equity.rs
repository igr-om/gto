use crossbeam::deque::{Injector, Steal, Stealer, Worker};
use tokio::sync::oneshot;
use rand::Rng;
use std::sync::Arc;
use std::thread;
use tracing::{debug, info, warn};
use crate::poker::{board::Board, cards::Card, deck::Deck};
use crate::variants::{nlhe, plo};

pub enum Task {
    NlheMonteCarlo {
        hand_id: u64,
        iterations: u32,
        hero_hole: [Card; 2],
        board: Board,
        respond_to: oneshot::Sender<ResultChunk>,
    },
    PloMonteCarlo {
        hand_id: u64,
        iterations: u32,
        hero_hole: [Card; 4],
        board: Board,
        respond_to: oneshot::Sender<ResultChunk>,
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
    _handles: Vec<thread::JoinHandle<()>>,
}

impl MonteCarloPool {
    pub fn new(num_workers: usize) -> Self {
        info!("initializing MonteCarloPool with {} workers", num_workers);
        let task_injector = Arc::new(Injector::new());

        let mut workers = Vec::new();
        let mut stealers = Vec::new();
        for _ in 0..num_workers {
            let w = Worker::new_fifo();
            stealers.push(w.stealer());
            workers.push(w);
        }

        let mut handles = Vec::new();
        for (id, local) in workers.into_iter().enumerate() {
            let global = task_injector.clone();
            let others = stealers.iter().filter(|s| !s.is_empty()).cloned().collect(); // Simplified for brevity
            
            let state = WorkerState { id, local, stealers: others, global };
            handles.push(thread::spawn(move || state.run()));
        }

        Self { task_injector, _handles: handles }
    }

    pub fn submit_task(&self, task: Task) {
        self.task_injector.push(task);
    }
}

struct WorkerState {
    id: usize,
    local: Worker<Task>,
    stealers: Vec<Stealer<Task>>,
    global: Arc<Injector<Task>>,
}

impl WorkerState {
    fn run(self) {
        let mut rng = rand::thread_rng();
        loop {
            if let Some(task) = self.local.pop().or_else(|| self.global.steal().success()) {
                self.handle_task(task, &mut rng);
            } else {
                break; // No more tasks
            }
        }
    }

    fn handle_task(&self, task: Task, rng: &mut impl Rng) {
        match task {
            Task::NlheMonteCarlo { hand_id, iterations, hero_hole, board, respond_to } => {
                let (wins, ties, total) = run_nlhe_monte_carlo(rng, hero_hole, &board, iterations);
                let _ = respond_to.send(ResultChunk { hand_id, wins, ties, total });
            }
            Task::PloMonteCarlo { hand_id, iterations, hero_hole, board, respond_to } => {
                let (wins, ties, total) = run_plo_monte_carlo(rng, hero_hole, &board, iterations);
                let _ = respond_to.send(ResultChunk { hand_id, wins, ties, total });
            }
            Task::DataAggregation => {
                warn!("DataAggregation not implemented");
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
) -> (u64, u64, u64) {
    let mut wins = 0;
    let mut ties = 0;
    let mut total = 0;

    for _ in 0..iterations {
        let mut deck = Deck::new();
        deck.remove_many(&hero_hole);
        deck.remove_many(&board.cards);

        let villain_hole = nlhe::sample_villain_hand(rng, &mut deck);

        let mut full_board = board.clone();
        full_board.complete_to_river(&mut deck, rng);

        let hero_rank = nlhe::evaluate_hand(&hero_hole, &full_board);
        let villain_rank = nlhe::evaluate_hand(&villain_hole, &full_board);

        if hero_rank > villain_rank {
            wins += 1;
        } else if hero_rank == villain_rank {
            ties += 1;
        }

        total += 1;
    }

    (wins, ties, total)
}

fn run_plo_monte_carlo<R: Rng>(
    rng: &mut R,
    hero_hole: [Card; 4],
    board: &Board,
    iterations: u32,
) -> (u64, u64, u64) {
    let mut wins = 0;
    let mut ties = 0;
    let mut total = 0;

    for _ in 0..iterations {
        let mut deck = Deck::new();
        deck.remove_many(&hero_hole);
        deck.remove_many(&board.cards);

        let villain_hole = {
            let c1 = deck.draw_random(rng);
            let c2 = deck.draw_random(rng);
            let c3 = deck.draw_random(rng);
            let c4 = deck.draw_random(rng);
            [c1, c2, c3, c4]
        };

        let mut full_board = board.clone();
        full_board.complete_to_river(&mut deck, rng);

        let hero_rank = plo::evaluate_hand(&hero_hole, &full_board);
        let villain_rank = plo::evaluate_hand(&villain_hole, &full_board);

        if hero_rank > villain_rank {
            wins += 1;
        } else if hero_rank == villain_rank {
            ties += 1;
        }

        total += 1;
    }

    (wins, ties, total)
}
