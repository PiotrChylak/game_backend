use url::Url;
use starknet::core::types::Felt;

#[derive(Clone)]
pub struct AppState {
    pub url: Url,
    pub sender_address: String,
    pub private_key: String,
}

pub const SEPOLIA: Felt = Felt::from_raw([
    507980251676163170,
    18446744073709551615,
    18446744073708869172,
    1555806712078248243,
]);
