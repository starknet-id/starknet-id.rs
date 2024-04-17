use starknet::{
    core::types::FieldElement,
    providers::{jsonrpc::HttpTransport, JsonRpcClient},
};
use starknet_id::{
    naming::{ResolvingError, SEPOLIA_CONTRACT},
    ProviderExt,
};
use url::Url;

fn create_jsonrpc_client() -> JsonRpcClient<HttpTransport> {
    let rpc_url =
        std::env::var("STARKNET_RPC").unwrap_or("https://sepolia.rpc.starknet.id/".into());
    JsonRpcClient::new(HttpTransport::new(Url::parse(&rpc_url).unwrap()))
}

#[tokio::main]
async fn main() {
    let client_sepolia = create_jsonrpc_client();
    println!("On sepolia:");
    let addr = client_sepolia
        .domain_to_address("th0rgal.stark", SEPOLIA_CONTRACT)
        .await;
    match addr {
        Ok(addr) => println!("address: 0x{:x}", addr),
        Err(err) => match err {
            ResolvingError::ConnectionError(cause) => println!("Connection error: {}", cause),
            ResolvingError::InvalidContractResult => println!("Invalid contract result"),
            ResolvingError::InvalidDomain => println!("Invalid domain"),
            ResolvingError::NotSupported => println!("Resolving not supported"),
        },
    }

    let domain_result = client_sepolia
        .address_to_domain(
            FieldElement::from_hex_be(
                "0x0403c80a49f16Ed8Ecf751f4B3Ad62CC8f85EbEB2d40DC3B4377a089b438995D",
            )
            .unwrap(),
            SEPOLIA_CONTRACT,
        )
        .await;
    match domain_result {
        Ok(domain_result) => println!("domain: {}", domain_result),
        Err(err) => match err {
            ResolvingError::ConnectionError(cause) => println!("Connection error: {}", cause),
            ResolvingError::InvalidContractResult => println!("Invalid contract result"),
            ResolvingError::InvalidDomain => println!("Invalid domain"),
            ResolvingError::NotSupported => println!("Resolving not supported"),
        },
    }
}
