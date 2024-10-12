use axum::extract::{Query, State};
use axum::{routing::get, Router};
use axum::response::{Html, Json};
use serde::{Deserialize, Serialize};
// use starknet::core::chain_id;
use starknet::providers::jsonrpc::HttpTransport;
use std::env::args;
use std::net::SocketAddr;
use std::str::FromStr;
use starknet::accounts::{Account, ConnectedAccount, ExecutionEncoding, SingleOwnerAccount}; // single_owner, 
use starknet::providers::{JsonRpcClient, Provider}; //Provider , SequencerGatewayProvider
use starknet::signers::{LocalWallet, SigningKey}; //Signer
use starknet::core::types::{Call, Felt};
use starknet::core::utils::get_selector_from_name;
// use starknet::contract::ContractFactory;
use url::Url;
use starknet::core::types::{FunctionCall, BlockId, BlockTag};

use std::path::PathBuf;
use clap::{arg, Parser};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, short, env)]
    pub url: Url,
    #[arg(long, short, env)]
    pub sender_address: String,
    #[arg(long, short, env)]
    pub private_key: String,
}

#[derive(Clone)]
pub struct AppState {
    pub url: Url,
    pub sender_address: String,
    pub private_key: String,
}

const CONTRACT_ADDRESS: &str = "0x4881106983c4e4fce51627cb3845995ea40ff68808bfb15dd1ad85915f05605";

pub const SEPOLIA: Felt = Felt::from_raw([
    507980251676163170,
    18446744073709551615,
    18446744073708869172,
    1555806712078248243,
]);

#[derive(Deserialize)]
struct TeleportParams {
    x: i64,
    y: i64,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // let app_state = AppState {
    //     message_expiration_time: args.message_expiration_time,
    //     session_expiration_time: args.session_expiration_time,
    //     jwt_secret_key: args.jwt_secret_key,
    //     nonces: Arc::new(Mutex::new(HashMap::new())),
    //     authorizer,
    //     job_store: JobStore::default(),
    //     thread_pool: Arc::new(Mutex::new(ThreadPool::new(args.num_workers))),
    //     admin_keys,
    //     sse_tx: Arc::new(Mutex::new(sse_tx)),
    // };

    let app_state = AppState{
        url: args.url.clone(),
        sender_address: args.sender_address.clone(),
        private_key: args.private_key.clone()
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/initialize", get(initialize_map))
        .route("/move_forward", get(move_forward))
        .route("/move_down", get(move_down))
        .route("/move_left", get(move_left))
        .route("/move_right", get(move_right))
        .route("/teleport_to", get(teleport_to)) //usage example: /teleport_to?x=10&y=20 (DO ZMIANY!) a moze jednak nie do zmiany
        .route("/get_position", get(get_position)).with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Html<&'static str> {
    Html(r#"
        <h1>Welcome to the StarkNet API Server</h1>
        <p>Try calling the API endpoints below:</p>
        <ul>
            <li><strong>/initialize</strong>: Initializes the game map.</li>
            <li><strong>/move_forward</strong>: Moves the player forward in the game.</li>
            <li><strong>/move_down</strong>: Moves the player down in the game.</li>
            <li><strong>/move_left</strong>: Moves the player left in the game.</li>
            <li><strong>/move_right</strong>: Moves the player right in the game.</li>
            <li><strong>/teleport_to</strong>: Teleports the player to given position.</li>
            <li><strong>/get_position</strong>: Retrieves the current position of the player.</li>
        </ul>
    "#)
}

async fn invoke_contract_method(method: &str, calldata: Vec<Felt>, url: Url, sender_address: String, private_key: String) -> Result<String, String> {
    let sender_address = Felt::from_hex(sender_address.as_str()).unwrap();
    let private_key = Felt::from_hex(private_key.as_str()).unwrap();
    
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(private_key));
    let provider = JsonRpcClient::new(HttpTransport::new(url));

    let chain_id = SEPOLIA;

    let account = SingleOwnerAccount::new(
        provider,
        signer,
        sender_address,
        chain_id,
        ExecutionEncoding::New
    );

    let contract = Felt::from_hex(CONTRACT_ADDRESS).unwrap();
    let selector = get_selector_from_name(method).unwrap();

    let nonce = account.get_nonce().await.unwrap();

    let tx = account
        .execute_v1(vec![Call {
            to: contract,
            selector,
            calldata,
        }])
        .nonce(nonce)
        .send()
        .await;

    match tx {
        Ok(_) => Ok("Transaction sent successfully!".to_string()),
        Err(e) => Err(format!("Error: {:?}", e)),
    }
}

// Function for calling read-only contract methods (e.g., `get_position`)
async fn call_contract_ro(method: &str, url: Url) -> (Felt, Felt) {
    let provider = JsonRpcClient::new(HttpTransport::new(url));

    let contract_address = Felt::from_hex(CONTRACT_ADDRESS).unwrap();
    let selector = get_selector_from_name(method).unwrap();

    let calldata: Vec<Felt> = vec![];

    let function_call = FunctionCall {
        contract_address,
        entry_point_selector: selector,
        calldata,
    };

    let result = provider
        .call(function_call, &BlockId::Tag(BlockTag::Latest))
        .await
        .unwrap();

    let x = result[0];
    let y = result[1];

    (x, y)
}

// Handlers for various contract methods

async fn initialize_map(State(state): State<AppState>) -> Json<ApiResponse> {
    let result = invoke_contract_method("initialize_map", vec![], state.url, state.sender_address, state.private_key).await;
    Json(ApiResponse {
        message: result.unwrap_or_else(|err| err),
    })
}

async fn move_forward(State(state): State<AppState>) -> Json<ApiResponse> {
    let result = invoke_contract_method("update_position", vec![Felt::from(1), Felt::from(0)], state.url, state.sender_address, state.private_key).await;
    Json(ApiResponse {
        message: result.unwrap_or_else(|err| err),
    })
}

async fn move_down(State(state): State<AppState>) -> Json<ApiResponse> {
    let result = invoke_contract_method("update_position", vec![Felt::from(-1), Felt::from(0)], state.url, state.sender_address, state.private_key).await;
    Json(ApiResponse {
        message: result.unwrap_or_else(|err| err),
    })
}

async fn move_left(State(state): State<AppState>) -> Json<ApiResponse> {
    let result = invoke_contract_method("update_position", vec![Felt::from(0), Felt::from(-1)], state.url, state.sender_address, state.private_key).await;
    Json(ApiResponse {
        message: result.unwrap_or_else(|err| err),
    })
}

async fn move_right(State(state): State<AppState>) -> Json<ApiResponse> {
    let result = invoke_contract_method("update_position", vec![Felt::from(0), Felt::from(1)], state.url, state.sender_address, state.private_key).await;
    Json(ApiResponse {
        message: result.unwrap_or_else(|err| err),
    })
}

async fn teleport_to(Query(params): Query<TeleportParams>, State(state): State<AppState>) -> Json<ApiResponse> {
    let result = invoke_contract_method("update_position",vec![Felt::from(params.x), Felt::from(params.y)], state.url, state.sender_address, state.private_key).await;

    Json(ApiResponse {
        message: result.unwrap_or_else(|err| err),
    })
}

async fn get_position(State(state): State<AppState>) -> Json<PositionResponse> {
    let position = call_contract_ro("get_position", state.url).await;
    Json(PositionResponse {
        x: position.0,
        y: position.1,
    })
}

// API response structure
#[derive(Serialize)]
struct ApiResponse {
    message: String,
}

#[derive(Serialize)]
struct PositionResponse {
    x: Felt,
    y: Felt,
}