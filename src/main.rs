mod poker;
mod variants;
mod compute;


use compute::monte_carlo::{MonteCarloPool, Task};

fn main() {
    let pool = MonteCarloPool::new(4);

    for i in 0..10 {
        pool.submit_task(Task::MonteCarloSample {
            hand_id: i,
            iterations: 10_000,
        });
    }

    for _ in 0..10 {
        if let Ok(chunk) = pool.results().recv() {
            let equity = (chunk.wins as f64 + 0.5 * chunk.ties as f64)
                / chunk.total as f64;
            println!(
                "hand {}: equity ~ {:.2}%",
                chunk.hand_id,
                equity * 100.0
            );
        }
    }
}
