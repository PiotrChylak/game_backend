use serde::{Deserialize, Serialize};
use starknet::core::types::Felt;

#[derive(Deserialize)]
pub struct TeleportParams {
    pub x: i64,
    pub y: i64,
}

#[derive(Deserialize)]
pub struct InitializeMapParams {
    pub coords: Vec<i64>,
}

#[derive(Serialize)]
pub struct ApiResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct PositionResponse {
    pub x: Felt,
    pub y: Felt,
}

#[derive(Serialize)]
pub struct WallPositionResponse {
    pub walls: Vec<(Felt, Felt)>,
}
