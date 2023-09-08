use crate::{
    decode, encode,
    naming::{ResolvingError, SELECTOR_A2D, SELECTOR_D2A},
};
use async_trait::async_trait;
use starknet::providers::sequencer::models::BlockId;
use starknet::{
    core::types::FieldElement,
    providers::{sequencer::models::CallFunction, SequencerGatewayProvider},
};

#[async_trait]
pub trait SequencerGatewayProviderExt {
    async fn domain_to_address(
        &self,
        domain: &str,
        contract_addr: FieldElement,
    ) -> Result<FieldElement, ResolvingError>;

    async fn address_to_domain(
        &self,
        address: FieldElement,
        contract_addr: FieldElement,
    ) -> Result<String, ResolvingError>;
}

// Implement the extension trait for SequencerGatewayProvider
#[async_trait]
impl SequencerGatewayProviderExt for SequencerGatewayProvider {
    async fn domain_to_address(
        &self,
        domain: &str,
        contract_addr: FieldElement,
    ) -> Result<FieldElement, ResolvingError> {
        if !domain.ends_with(".stark") {
            return Err(ResolvingError::InvalidDomain);
        }
        let encoding_result = encode(&domain[0..domain.len() - 6]);
        match encoding_result {
            Ok(encoded) => {
                match self
                    .call_contract(
                        CallFunction {
                            contract_address: contract_addr,
                            entry_point_selector: SELECTOR_D2A,
                            calldata: vec![FieldElement::ONE, encoded],
                        },
                        BlockId::Latest,
                    )
                    .await
                {
                    Ok(result) => match result.result.first() {
                        Some(x) => Ok(*x),
                        None => Err(ResolvingError::InvalidContractResult),
                    },
                    Err(cause) => Err(ResolvingError::ConnectionError(cause.to_string())),
                }
            }
            Err(_) => Err(ResolvingError::InvalidDomain),
        }
    }

    async fn address_to_domain(
        &self,
        address: FieldElement,
        contract_addr: FieldElement,
    ) -> Result<String, ResolvingError> {
        match self
            .call_contract(
                CallFunction {
                    contract_address: contract_addr,
                    entry_point_selector: SELECTOR_A2D,
                    calldata: vec![address],
                },
                BlockId::Latest,
            )
            .await
        {
            Ok(result) => {
                let mut calldata = result.result.iter();
                let mut domain = String::new().to_owned();
                match calldata.next() {
                    Some(_) => {
                        calldata.for_each(|value| {
                            domain.push_str(decode(*value).as_str());
                            domain.push('.');
                        });
                        domain.push_str("stark");
                        Ok(domain)
                    }
                    None => Err(ResolvingError::InvalidContractResult),
                }
            }
            Err(cause) => Err(ResolvingError::ConnectionError(cause.to_string())),
        }
    }
}
