mod poker;
mod compute;
mod variants;
mod ux;

use std::sync::{Arc, Mutex};
use std::str::FromStr;

use tera::Tera;
use axum::{Router, routing::{get, post}, serve, extract::State, Json};
use serde::Deserialize;
use serde::Serialize;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use ux::controllers::{index, run};
use crate::poker::{
    cards::Card,
    gto_range::RangeTable,
    user_stats::UserStats,
    drill_result::DrillResult,
    drill::Drill,
    adaptive::AdaptiveEngine,
};
use rand::thread_rng;

#[derive(Deserialize)]
pub struct SubmitPayload {
    pub position: String,
    pub action: String,
    pub user_action: String,
    pub hand: Vec<Card>,
}

#[derive(Clone)]
pub struct AppState {
    pub gto: Arc<RangeTable>,
    pub stats: Arc<Mutex<UserStats>>,
    pub tera: Arc<Tera>,
}

#[derive(Serialize)]
pub struct StatsResponse {
    pub total: u32,
    pub correct: u32,
    pub accuracy: f64,
}

pub async fn get_stats(State(state): State<AppState>) -> Json<StatsResponse> {
    let stats = state.stats.lock().unwrap();

    let mut total = 0;
    let mut correct = 0;

    for ((_pos, _act), (c, t)) in stats.stats.iter() {
        correct += *c;
        total += *t;
    }

    let accuracy = if total == 0 {
        0.0
    } else {
        correct as f64 / total as f64
    };

    Json(StatsResponse {
        total,
        correct,
        accuracy,
    })
}

pub async fn submit_drill(
    State(state): State<AppState>,
    Json(payload): Json<SubmitPayload>,
) -> Json<DrillResult> {
    use crate::poker::gto_range::{Position, Action};

    let pos = Position::from_str(&payload.position).unwrap();
    let act = Action::from_str(&payload.action).unwrap();
    let user_act = Action::from_str(&payload.user_action).unwrap();

    let correct = user_act == act;

    {
        let mut stats = state.stats.lock().unwrap();
        stats.record(pos, act, correct);
    }

    Json(DrillResult {
        position: pos,
        action: act,
        hand: payload.hand,
        user_action: user_act,
        correct,
    })
}

pub async fn next_drill(
    State(state): State<AppState>,
) -> Json<Drill> {
    let mut rng = thread_rng();

    let stats = state.stats.lock().unwrap();
    let (pos, act) = AdaptiveEngine::next_action(&mut rng, &stats);
    drop(stats);

    let drill = Drill::generate(&mut rng, &state.gto, pos, act)
        .expect("No range for this drill");

    Json(drill)
}

pub async fn get_drill(
    State(state): State<AppState>,
) -> Json<Drill> {
    let mut rng = thread_rng();

    let stats = state.stats.lock().unwrap();
    let (pos, act) = AdaptiveEngine::next_action(&mut rng, &stats);
    drop(stats);

    let drill = Drill::generate(&mut rng, &state.gto, pos, act)
        .expect("No range for this drill");

    Json(drill)
}

#[tokio::main]
async fn main() {
    let tera = Arc::new(Tera::new("src/ux/templates/**/*").unwrap());

    let state = AppState {
        gto: Arc::new(RangeTable::from_csv("ranges/preflop.csv")),
        stats: Arc::new(Mutex::new(UserStats::new())),
        tera: tera.clone(),
    };

    let app = Router::new()
        .route("/", get(index))
        .route("/run", post(run))
        .route("/drill", get(get_drill))
        .route("/drill/submit", post(submit_drill))
        .route("/drill/next", get(next_drill))
        .route("/stats", get(get_stats))
        .nest_service("/static", ServeDir::new("src/ux/static"))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}
