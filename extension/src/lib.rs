use pallas::ledger::{
    addresses::{Address, ShelleyAddress},
    primitives::ToCanonicalJson,
    traverse::{MultiEraBlock, MultiEraTx},
};
use pgrx::prelude::*;

pgrx::pg_module_magic!();

#[pg_extern]
fn hello_extension() -> &'static str {
    "Hello, extension"
}

#[pg_extern]
fn pretty_cbor(bytes: &[u8]) -> String {
    let tokens = pallas::codec::minicbor::decode::Tokenizer::new(bytes);
    format!("{}", tokens)
}

#[pg_extern]
fn block_hash(bytes: &[u8]) -> String {
    let block = MultiEraBlock::decode(bytes).unwrap();
    block.hash().to_string()
}

#[pg_extern]
fn tx_has_address(tx_cbor: &[u8], address: &str) -> bool {
    let expected = match Address::from_bech32(address) {
        Ok(x) => x,
        Err(_) => return false,
    };

    let tx = match MultiEraTx::decode(tx_cbor) {
        Ok(x) => x,
        Err(_) => return false,
    };

    tx.outputs()
        .iter()
        .map(|x| x.address().ok())
        .flatten()
        .any(|x| x == expected)
}

#[pg_extern]
fn tx_output_addresses(tx_cbor: &[u8]) -> Vec<Vec<u8>> {
    let tx = match MultiEraTx::decode(tx_cbor) {
        Ok(x) => x,
        Err(_) => return vec![],
    };

    tx.outputs()
        .iter()
        .map(|x| x.address().ok())
        .flatten()
        .map(|x| x.to_vec())
        .collect()
}

#[pg_extern]
fn tx_has_input(tx_cbor: &[u8], hash: &[u8], index: Option<i32>) -> bool {
    let tx = match MultiEraTx::decode(tx_cbor) {
        Ok(x) => x,
        Err(_) => return false,
    };

    tx.inputs().iter().any(|i| i.hash().as_ref().eq(hash))
}

#[pg_extern]
fn tx_has_reference_input(tx_cbor: &[u8], hash: &[u8], index: Option<i32>) -> bool {
    let tx = match MultiEraTx::decode(tx_cbor) {
        Ok(x) => x,
        Err(_) => return false,
    };

    tx.reference_inputs()
        .iter()
        .any(|i| i.hash().as_ref().eq(hash))
}

#[pg_extern]
fn tx_reference_inputs(tx_cbor: &[u8]) -> Vec<Vec<u8>> {
    let tx = match MultiEraTx::decode(tx_cbor) {
        Ok(x) => x,
        Err(_) => return vec![],
    };

    tx.reference_inputs()
        .iter()
        .map(|i| i.hash().to_vec())
        .collect()
}

#[pg_extern]
fn tx_redeemers(tx_cbor: &[u8]) -> Vec<pgrx::Json> {
    let tx = match MultiEraTx::decode(tx_cbor) {
        Ok(x) => x,
        Err(_) => return vec![],
    };

    tx.redeemers()
        .iter()
        .map(|x| pgrx::Json(x.data.to_json()))
        .collect()
}

#[pg_extern]
fn tx_plutus_data(tx_cbor: &[u8]) -> Vec<pgrx::Json> {
    let tx = match MultiEraTx::decode(tx_cbor) {
        Ok(x) => x,
        Err(_) => return vec![],
    };

    tx.plutus_data()
        .iter()
        .map(|x| pgrx::Json(x.to_json()))
        .collect()
}

#[pg_extern]
fn tx_has_redeemer(tx_cbor: &[u8]) -> bool {
    let tx = match MultiEraTx::decode(tx_cbor) {
        Ok(x) => x,
        Err(_) => return false,
    };

    tx.redeemers().iter().count() > 0
}

#[pg_extern]
fn tx_output_addresses_bech32(tx_cbor: &[u8]) -> Vec<String> {
    let tx = match MultiEraTx::decode(tx_cbor) {
        Ok(x) => x,
        Err(_) => return vec![],
    };

    tx.outputs()
        .iter()
        .map(|x| x.address().ok())
        .flatten()
        .map(|x| x.to_bech32().ok())
        .flatten()
        .collect()
}

#[pg_extern]
fn address_payment_part(address: &[u8]) -> Option<String> {
    let addr = match Address::from_bytes(address) {
        Ok(x) => x,
        Err(_) => return None,
    };

    match addr {
        Address::Shelley(addr) => addr.payment().to_bech32().ok(),
        _ => None,
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_hello_extension() {
        assert_eq!("Hello, extension", crate::hello_extension());
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
