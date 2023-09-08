use crate::{
    decode, encode,
    naming::{ResolvingError, SELECTOR_A2D, SELECTOR_D2A},
};
use async_trait::async_trait;
use starknet::core::types::FieldElement;
use starknet::{
    core::types::{BlockId, BlockTag, FunctionCall},
    providers::Provider,
};

#[async_trait]
pub trait ProviderExt {
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

#[async_trait]
impl<T: Provider + Sync> ProviderExt for T {
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
                    .call(
                        FunctionCall {
                            contract_address: contract_addr,
                            entry_point_selector: SELECTOR_D2A,
                            calldata: vec![FieldElement::ONE, encoded],
                        },
                        BlockId::Tag(BlockTag::Latest),
                    )
                    .await
                {
                    Ok(result) => match result.first() {
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
            .call(
                FunctionCall {
                    contract_address: contract_addr,
                    entry_point_selector: SELECTOR_A2D,
                    calldata: vec![address],
                },
                BlockId::Tag(BlockTag::Latest),
            )
            .await
        {
            Ok(result) => {
                let mut calldata = result.iter();
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
