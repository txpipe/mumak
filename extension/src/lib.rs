use pallas::ledger::{
    addresses::Address,
    traverse::{MultiEraBlock, MultiEraTx},
};
use pgrx::prelude::*;

pgrx::pg_module_magic!();

#[pg_extern]
fn hello_extension() -> &'static str {
    "Hello, extension"
}

#[pg_extern]
fn block_hash(bytes: &[u8]) -> String {
    let block = MultiEraBlock::decode(bytes).unwrap();
    block.hash().to_string()
}

#[pg_extern]
fn tx_has_address(tx_cbor: &[u8], tx_era: u32, address: &[u8]) -> bool {
    let expected = match Address::from_bytes(address) {
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
fn tx_output_addresses(tx_cbor: &[u8], tx_era: u32) -> Vec<Vec<u8>> {
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
