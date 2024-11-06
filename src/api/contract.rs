use starknet::accounts::{Account, ConnectedAccount, ExecutionEncoding, SingleOwnerAccount};
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider};
use starknet::signers::{LocalWallet, SigningKey};
use starknet::core::types::{Call, Felt, BlockId, BlockTag, FunctionCall};
use starknet::core::utils::get_selector_from_name;
use url::Url;
use crate::state::SEPOLIA;

pub const CONTRACT_ADDRESS: &str = "0x2c5ecb4bd05fb50fc0da17a804a4e9fa22272796c4e942b5d45d5513ea3888e";

pub async fn invoke_contract_method(method: &str, calldata: Vec<Felt>, url: Url, sender_address: String, private_key: String) -> Result<String, String> {
    let sender_address = Felt::from_hex(sender_address.as_str()).unwrap();
    let private_key = Felt::from_hex(private_key.as_str()).unwrap();
    
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(private_key));
    let provider = JsonRpcClient::new(HttpTransport::new(url));

    let account = SingleOwnerAccount::new(
        provider,
        signer,
        sender_address,
        SEPOLIA,
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

pub async fn call_contract_ro_vec(method: &str, url: Url) -> Vec<Felt> {
    let provider = JsonRpcClient::new(HttpTransport::new(url));
    let contract_address = Felt::from_hex(CONTRACT_ADDRESS).unwrap();
    let selector = get_selector_from_name(method).unwrap();
    let function_call = FunctionCall {
        contract_address,
        entry_point_selector: selector,
        calldata: vec![],
    };

    provider
        .call(function_call, &BlockId::Tag(BlockTag::Latest))
        .await
        .unwrap()
}

pub async fn call_contract_ro_tuple(method: &str, url: Url) -> (Felt, Felt) {
    let result = call_contract_ro_vec(method, url).await;
    (result[0], result[1])
}
