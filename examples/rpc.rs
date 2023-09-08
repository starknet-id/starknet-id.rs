use starknet::{
    core::types::FieldElement,
    providers::{jsonrpc::HttpTransport, JsonRpcClient},
};
use starknet_id::{
    naming::{ResolvingError, GOERLI_CONTRACT},
    ProviderExt,
};
use url::Url;

fn create_jsonrpc_client() -> JsonRpcClient<HttpTransport> {
    let rpc_url =
        std::env::var("STARKNET_RPC").unwrap_or("https://rpc-goerli-1.starknet.rs/rpc/v0.4".into());
    JsonRpcClient::new(HttpTransport::new(Url::parse(&rpc_url).unwrap()))
}

#[tokio::main]
async fn main() {
    let client_goerli = create_jsonrpc_client();
    println!("On goerli:");
    let addr = client_goerli
        .domain_to_address("th0rgal.stark", GOERLI_CONTRACT)
        .await;
    match addr {
        Ok(addr) => println!("address: {}", addr),
        Err(err) => match err {
            ResolvingError::ConnectionError(cause) => println!("Connection error: {}", cause),
            ResolvingError::InvalidContractResult => println!("Invalid contract result"),
            ResolvingError::InvalidDomain => println!("Invalid domain"),
            ResolvingError::NotSupported => println!("Resolving not supported"),
        },
    }

    let domain_result = client_goerli
        .address_to_domain(
            FieldElement::from_hex_be(
                "0x048F24D0D0618fa31813DB91a45d8be6c50749e5E19ec699092CE29aBe809294",
            )
            .unwrap(),
            GOERLI_CONTRACT,
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
