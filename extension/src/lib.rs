use pallas::ledger::addresses::Address;
use pallas::ledger::addresses::ByronAddress;
use pallas::ledger::traverse::MultiEraTx;
use pgrx::prelude::*;

pgrx::pg_module_magic!();

#[pg_extern]
fn hello_extension() -> &'static str {
    "Hello, extension"
}

#[pg_extern]
fn tx_hash(tx_cbor: &[u8]) -> String {
    let tx = match MultiEraTx::decode(tx_cbor) {
        Ok(x) => x,
        Err(_) => return "".to_string(),
    };

    tx.hash().to_string()
}

#[pg_extern]
fn tx_inputs(tx_cbor: &[u8]) -> TableIterator<'static, (name!(hash, String), name!(index, i64))> {
    let tx = match MultiEraTx::decode(tx_cbor) {
        Ok(x) => x,
        Err(_) => return TableIterator::new(std::iter::empty()),
    };

    let inputs_data = tx
        .inputs()
        .iter()
        .map(|i| (i.hash().to_string(), i.index() as i64))
        .collect::<Vec<_>>();

    TableIterator::new(inputs_data.into_iter())
}

#[pg_extern]
fn tx_inputs_json(tx_cbor: &[u8]) -> pgrx::Json {
    let tx = match MultiEraTx::decode(tx_cbor) {
        Ok(x) => x,
        Err(_) => return pgrx::Json(serde_json::to_value(vec![""]).unwrap()),
    };

    pgrx::Json(
        serde_json::to_value(
            tx.inputs()
                .iter()
                .map(|i| (i.hash().to_string(), i.index()))
                .collect::<Vec<(String, u64)>>(),
        )
        .unwrap(),
    )
}

#[pg_extern]
fn tx_inputs_cbor(tx_cbor: &[u8]) -> Vec<u8> {
    let tx = match MultiEraTx::decode(tx_cbor) {
        Ok(x) => x,
        Err(_) => return vec![],
    };

    let inputs = tx
        .inputs()
        .iter()
        .map(|i| (i.hash().to_string(), i.index()))
        .collect::<Vec<_>>();
    let mut encoded_inputs: Vec<u8> = Vec::new();
    pallas::codec::minicbor::encode(&inputs, &mut encoded_inputs).unwrap();
    encoded_inputs
}

#[pg_extern]
fn tx_addresses(tx_cbor: &[u8]) -> SetOfIterator<'static, Vec<u8>> {
    let tx = match MultiEraTx::decode(tx_cbor) {
        Ok(x) => x,
        Err(_) => return SetOfIterator::new(std::iter::empty()),
    };

    let outputs_data = tx
        .outputs()
        .iter()
        .map(|o| o.address().unwrap().to_vec())
        .collect::<Vec<_>>();

    SetOfIterator::new(outputs_data)
}

#[pg_extern]
fn tx_addresses_json(tx_cbor: &[u8]) -> pgrx::Json {
    let tx = match MultiEraTx::decode(tx_cbor) {
        Ok(x) => x,
        Err(_) => return pgrx::Json(serde_json::to_value(vec![""]).unwrap()),
    };

    pgrx::Json(
        serde_json::to_value(
            tx.outputs()
                .iter()
                .map(|o| match o.address().unwrap().to_bech32() {
                    Ok(address) => address,
                    Err(_) => ByronAddress::from_bytes(&o.address().unwrap().to_vec())
                        .unwrap()
                        .to_base58(),
                })
                .collect::<Vec<String>>(),
        )
        .unwrap(),
    )
}

#[pg_extern]
fn address_network_id(address: &[u8]) -> i64 {
    let address = match Address::from_bytes(address) {
        Ok(x) => x,
        Err(_) => return -1,
    };

    let network_id = match address.network() {
        Some(n) => n.value() as i64,
        None => -1,
    };

    network_id
}

#[pg_extern]
fn address_payment_part(address: &[u8]) -> Vec<u8> {
    let address = match Address::from_bytes(address) {
        Ok(x) => x,
        Err(_) => return vec![],
    };

    let payment_part = match address {
        Address::Shelley(a) => a.payment().to_vec(),
        Address::Byron(a) => {
            vec![]
        }
        _ => return vec![],
    };

    payment_part
}

#[pg_extern]
fn address_stake_part(address: &[u8]) -> Vec<u8> {
    let address = match Address::from_bytes(address) {
        Ok(x) => x,
        Err(_) => return vec![],
    };

    let stake_part = match address {
        Address::Shelley(a) => a.delegation().to_vec(),
        Address::Byron(a) => {
            vec![]
        }
        _ => return vec![],
    };

    stake_part
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
