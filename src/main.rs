use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use rs_poker::core::{Hand, Suit, Value};
use rs_poker::holdem::MonteCarloGame;
use rs_poker::{self, core::Card};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/poker", get(poker))
        .route("/users", post(create_user));

    let addr = format!("{}:{}", "0.0.0.0", "3000");
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, urindanger2!"
}

async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    return (StatusCode::CREATED, Json(user));
}

async fn poker() -> (StatusCode, String) {
    let hands = ["Adkh", "8c8s"]
        .iter()
        .map(|s| Hand::new_from_str(s).expect("Should be able to create a hand."))
        .collect();

    let flop_card1 = Card {
        value: Value::Three,
        suit: Suit::Spade,
    };
    let flop_card2 = Card {
        value: Value::Four,
        suit: Suit::Spade,
    };
    let flop_card3 = Card {
        value: Value::Four,
        suit: Suit::Club,
    };

    let _board = [flop_card1, flop_card2, flop_card3];

    let mut g = MonteCarloGame::new_with_hands(hands).expect("Should be able to create a game.");

    let mut wins: [u64; 2] = [0, 0];
    for _ in 0..2_000_000 {
        let r = g.simulate().expect("Should be able to simulate a hand.");
        g.reset();
        wins[r.0] += 1
    }

    let w1 = wins[0] as f64;
    let w2 = wins[1] as f64;
    let perc: f64 = w1 / ((w1 + w2) / 100.0);
    let wins_string = wins
        .iter()
        .map(|&num| num.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    println!("{}", wins_string);
    return (StatusCode::CREATED, format!("{}", perc));
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
