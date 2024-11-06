use axum::extract::{Query, State};
use axum::response::{Html, Json};
use crate::state::AppState;
use super::contract::{invoke_contract_method, call_contract_ro_tuple, call_contract_ro_vec};
use super::models::{ApiResponse, PositionResponse, WallPositionResponse, InitializeMapParams, TeleportParams};
use starknet::core::types::Felt;

pub async fn root() -> Html<&'static str> {
    Html(r#"
        <h1>Welcome to the StarkNet API Server</h1>
        <p>Use the buttons below to call the API endpoints:</p>
        <button onclick="callApi('/move_forward')">Move Forward</button>
        <button onclick="callApi('/move_down')">Move Down</button>
        <button onclick="callApi('/move_left')">Move Left</button>
        <button onclick="callApi('/move_right')">Move Right</button>
        <button onclick="promptTeleport()">Teleport to</button>
        <button onclick="callApi('/get_position')">Get Position</button>
        <button onclick="resetPosition()">Reset Position to (1, 1)</button>
        <button onclick="promptAddWalls()">Add Walls</button>
        <button onclick="callApi('/get_wall_positions')">Get Wall Positions</button>

        <div id="response"></div>

        <script>
            document.addEventListener('DOMContentLoaded', () => {
                console.log("Document fully loaded and parsed.");
            });

            async function callApi(endpoint) {
                console.log(`Calling API: ${endpoint}`);
                try {
                    const response = await fetch(endpoint);
                    const data = await response.json();
                    document.getElementById('response').innerText = JSON.stringify(data, null);
                } catch (error) {
                    console.error('Error calling API:', error);
                }
            }

            async function promptAddWalls() {
                const wallPoints = prompt("Enter wall points as x1,y1,x2,y2,... (pairs of coordinates):");
                if (wallPoints) {
                    const coordinates = wallPoints.split(',').map(Number);

                    console.log("Coordinates entered:", coordinates);

                    // Make sure to check if the coordinates are even (pairs)
                    if (coordinates.length % 2 !== 0) {
                        alert("Please enter an even number of coordinates.");
                        return;
                    }

                    const query = coordinates.map(coord => `coords[]=${coord}`).join('&');
                    const url = `/initialize_map?${query}`;

                    // Call the API with the constructed URL
                    await callApi(url);
                }
            }

            async function promptTeleport() {
                const x = prompt("Enter x coordinate:");
                const y = prompt("Enter y coordinate:");
                if (x !== null && y !== null) {
                    callApi(`/teleport_to?x=${x}&y=${y}`);
                }
            }

            function resetPosition() {
                callApi('/initialize_position');
            }
        </script>
    "#)
}

// Add other handlers like `initialize_map`, `move_forward`, etc., below...
pub async fn initialize_map(Query(params): Query<InitializeMapParams>, State(state): State<AppState>) -> Json<ApiResponse> {
    let wall_points: Vec<Felt> = params.coords.iter().map(|&coord| Felt::from(coord)).collect();
    println!("wall points: {:?}", wall_points);
    let result = invoke_contract_method("initialize_map", wall_points, state.url, state.sender_address, state.private_key).await;
    Json(ApiResponse {
        message: result.unwrap_or_else(|err| err),
    })
}

pub async fn initialize_position(State(state): State<AppState>) -> Json<ApiResponse> {
    let result = invoke_contract_method("initialize_position", vec![], state.url, state.sender_address, state.private_key).await;
    Json(ApiResponse {
        message: result.unwrap_or_else(|err| err),
    })
}

pub async fn move_forward(State(state): State<AppState>) -> Json<ApiResponse> {
    let result = invoke_contract_method("update_position", vec![Felt::from(0), Felt::from(1)], state.url, state.sender_address, state.private_key).await;
    Json(ApiResponse {
        message: result.unwrap_or_else(|err| err),
    })
}

pub async fn move_down(State(state): State<AppState>) -> Json<ApiResponse> {
    let result = invoke_contract_method("update_position", vec![Felt::from(0), Felt::from(-1)], state.url, state.sender_address, state.private_key).await;
    Json(ApiResponse {
        message: result.unwrap_or_else(|err| err),
    })
}

pub async fn move_left(State(state): State<AppState>) -> Json<ApiResponse> {
    let result = invoke_contract_method("update_position", vec![Felt::from(-1), Felt::from(0)], state.url, state.sender_address, state.private_key).await;
    Json(ApiResponse {
        message: result.unwrap_or_else(|err| err),
    })
}

pub async fn move_right(State(state): State<AppState>) -> Json<ApiResponse> {
    let result = invoke_contract_method("update_position", vec![Felt::from(1), Felt::from(0)], state.url, state.sender_address, state.private_key).await;
    Json(ApiResponse {
        message: result.unwrap_or_else(|err| err),
    })
}

pub async fn teleport_to(Query(params): Query<TeleportParams>, State(state): State<AppState>) -> Json<ApiResponse> {
    let result = invoke_contract_method("teleport",vec![Felt::from(params.x), Felt::from(params.y)], state.url, state.sender_address, state.private_key).await;

    Json(ApiResponse {
        message: result.unwrap_or_else(|err| err),
    })
}

pub async fn get_position(State(state): State<AppState>) -> Json<PositionResponse> {
    let position = call_contract_ro_tuple("get_position", state.url).await;
    Json(PositionResponse {
        x: position.0,
        y: position.1,
    })
}

pub async fn get_wall_positions(State(state): State<AppState>) -> Json<WallPositionResponse> {
    let mut wall_positions = call_contract_ro_vec("get_wall_positions", state.url).await; // Returns Vec<Felt>
    wall_positions.remove(0);

    let mut walls: Vec<(Felt, Felt)> = Vec::new();

    for chunk in wall_positions.chunks(2) {
        if chunk.len() == 2 {
            let x = chunk[0];
            let y = chunk[1];
            walls.push((x, y));
        }
    }
    Json(WallPositionResponse { walls })
}