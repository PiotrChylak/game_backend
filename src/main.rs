use axum::{routing::get, Router};
use std::net::SocketAddr;
use clap::Parser;

mod state;
mod config;
mod api;

use state::AppState;
use api::handlers::*;

#[tokio::main]
async fn main() {
    let args = config::Args::parse();

    let app_state = AppState {
        url: args.url.clone(),
        sender_address: args.sender_address.clone(),
        private_key: args.private_key.clone(),
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/initialize_position", get(initialize_position))
        .route("/initialize_map", get(initialize_map))
        .route("/move_forward", get(move_forward))
        .route("/move_down", get(move_down))
        .route("/move_left", get(move_left))
        .route("/move_right", get(move_right))
        .route("/teleport_to", get(teleport_to))
        .route("/get_wall_positions", get(get_wall_positions))
        .route("/get_position", get(get_position))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
