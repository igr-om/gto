use axum::{extract::{Form, State}, response::Html};
use serde::Deserialize;
use std::sync::Arc;
use tera::{Context, Tera};

use crate::compute::monte_carlo::MonteCarloPool;
use crate::poker::{
    board::Board,
    cards::Card,
    parse::{parse_hand, parse_board, parse_simple_range},
};

#[derive(Deserialize, Debug)]
pub struct RunForm {
    pub hero: String,
    pub board: String,
    pub range: String,
    pub variant: String,
}

pub async fn index(State(tera): State<Arc<Tera>>) -> Html<String> {
    let ctx = Context::new();
    let rendered = tera.render("index.html", &ctx).unwrap();
    Html(rendered)
}

pub async fn run(
    State(tera): State<Arc<Tera>>,
    Form(form): Form<RunForm>
) -> Html<String> {
    println!("FORM RECEIVED: {:?}", form);

    let hero_cards = parse_hand(&form.hero);
    let board_cards = parse_board(&form.board);
    let villain_range = parse_simple_range(&form.range);

    println!("HERO_CARDS: {:?} (len={})", hero_cards, hero_cards.len());
    println!("BOARD_CARDS: {:?} (len={})", board_cards, board_cards.len());
    println!("VILLAIN_RANGE HAND COUNT: {}", villain_range.hands.len());

    // Handle invalid hero hand
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
