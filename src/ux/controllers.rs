use axum::{
    extract::{Form, Query, State, Path},
    response::Html,
    Json,
};
use serde::Deserialize;
use std::str::FromStr;
use rand::Rng;

use tera::Context;

use crate::compute::monte_carlo::MonteCarloPool;
use crate::AppState;
use crate::poker::{
    board::Board,
    cards::Card,
    parse::{parse_hand, parse_board, parse_simple_range},
    gto_range::{Position, Action, expand_hand_to_combos, ComboRange},
    util::{hand_label, random_hand},
    drill::Drill,
    range::Range,
};

/* ============================
   HEATMAP ENDPOINT
   ============================ */

#[derive(Deserialize)]
pub struct HeatmapQuery {
    pub position: String,
    pub action: Option<String>,
}

#[derive(serde::Serialize)]
pub struct HeatmapResponse {
    pub matrix: Vec<Vec<f64>>,
    pub hands: Vec<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct OddsResponse {
    pub hand: String,
    pub vs_random: f64,
}

pub async fn odds_for_hand(
    State(_state): State<AppState>,
    Path(hand): Path<String>,
) -> Json<OddsResponse> {
    let combos = expand_hand_to_combos(&hand);

    let mut wins = 0;
    let mut ties = 0;
    let mut total = 0;

    let pool = MonteCarloPool::new(4);

    for combo in combos {
        let hero = [combo[0], combo[1]];

        for _ in 0..2000 {
            let villain_hand = random_hand(&hero);
            let villain_range = Range::from_hand(villain_hand);

            let result = pool.run_nlhe(
                hero,
                Board { cards: vec![] },
                villain_range,
                1,
            );

            wins += result.wins;
            ties += result.ties;
            total += result.total;
        }
    }

    let equity = if total > 0 {
        (wins as f64 + ties as f64 * 0.5) / total as f64
    } else {
        0.0
    };

    Json(OddsResponse {
        hand,
        vs_random: equity,
    })
}

/* ============================
   RANGE HEATMAP
   ============================ */

pub async fn range_heatmap(
    State(state): State<AppState>,
    Query(q): Query<HeatmapQuery>,
) -> Json<HeatmapResponse> {
    let pos = Position::from_str(&q.position).unwrap_or(Position::BTN);

    let act = q.action
        .as_deref()
        .and_then(|s| Action::from_str(s).ok())
        .unwrap_or(Action::Open);

    let combo = state.gto
        .to_combo_range(pos, act)
        .expect("No range for this position/action");

    let mut matrix = vec![vec![0.0; 13]; 13];
    let mut hands = vec![vec![String::new(); 13]; 13];

    for r in 0..13 {
        for c in 0..13 {
            let hand = hand_label(r, c);
            hands[r][c] = hand.clone();

            let freq = combo.frequency_for_hand(&hand).unwrap_or(0.0);
            matrix[r][c] = freq;
        }
    }

    Json(HeatmapResponse { matrix, hands })
}

/* ============================
   INDEX PAGE
   ============================ */

pub async fn index(State(state): State<AppState>) -> Html<String> {
    let tera = &state.tera;
    let ctx = Context::new();
    let rendered = tera.render("index.html", &ctx).unwrap();
    Html(rendered)
}

/* ============================
   EQUITY CALCULATION
   ============================ */

#[derive(Deserialize, Debug)]
pub struct RunForm {
    pub hero: String,
    pub board: String,
    pub range: String,
    pub variant: String,
}

pub async fn run(
    State(state): State<AppState>,
    Form(form): Form<RunForm>,
) -> Html<String> {
    let tera = &state.tera;

    let hero_cards = parse_hand(&form.hero);
    let board_cards = parse_board(&form.board);
    let villain_range = parse_simple_range(&form.range);

    if hero_cards.is_empty() {
        let mut ctx = Context::new();
        ctx.insert("wins", &0u64);
        ctx.insert("ties", &0u64);
        ctx.insert("total", &0u64);
        ctx.insert("error", &"Invalid hero hand".to_string());
        return Html(tera.render("results.html", &ctx).unwrap());
    }

    let board = Board { cards: board_cards };
    let pool = MonteCarloPool::new(4);

    let result = match form.variant.as_str() {
        "plo" => {
            if hero_cards.len() < 4 {
                let mut ctx = Context::new();
                ctx.insert("wins", &0u64);
                ctx.insert("ties", &0u64);
                ctx.insert("total", &0u64);
                ctx.insert("error", &"PLO requires 4 cards".to_string());
                return Html(tera.render("results.html", &ctx).unwrap());
            }

            let mut arr = [Card { rank: 0, suit: 'x' }; 4];
            for (i, c) in hero_cards.iter().take(4).enumerate() {
                arr[i] = *c;
            }

            pool.run_plo(arr, board, villain_range, 10_000)
        }

        _ => {
            if hero_cards.len() < 2 {
                let mut ctx = Context::new();
                ctx.insert("wins", &0u64);
                ctx.insert("ties", &0u64);
                ctx.insert("total", &0u64);
                ctx.insert("error", &"NLHE requires 2 cards".to_string());
                return Html(tera.render("results.html", &ctx).unwrap());
            }

            let mut arr = [Card { rank: 0, suit: 'x' }; 2];
            for (i, c) in hero_cards.iter().take(2).enumerate() {
                arr[i] = *c;
            }

            pool.run_nlhe(arr, board, villain_range, 10_000)
        }
    };

    let mut ctx = Context::new();
    ctx.insert("wins", &result.wins);
    ctx.insert("ties", &result.ties);
    ctx.insert("total", &result.total);
    ctx.insert("error", &Option::<String>::None);

    Html(tera.render("results.html", &ctx).unwrap())
}

/* ============================
   CLICKABLE HEATMAP → DRILL
   ============================ */

pub async fn drill_for_hand(
    State(state): State<AppState>,
    Path((hand, position)): Path<(String, String)>,
) -> Json<Drill> {
    let pos = Position::from_str(&position).unwrap_or(Position::BTN);
    let act = Action::Open;

    let combos = expand_hand_to_combos(&hand);

    let mut rng = rand::thread_rng();
    let cards = combos[rng.gen_range(0..combos.len())].clone();

    let drill = Drill {
        position: pos,
        action: act,
        hand: cards.iter().cloned().collect(),
    };

    Json(drill)
}