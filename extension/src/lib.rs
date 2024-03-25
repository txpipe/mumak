use pallas::ledger::traverse::MultiEraTx;
use pgrx::prelude::*;
use base64::prelude::*;

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
                .map(|i| (BASE64_STANDARD.encode(i.hash().to_vec()), i.index()))
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

    match tx {
        MultiEraTx::AlonzoCompatible(x, _) => {
            let mut encoded_inputs: Vec<u8> = Vec::new();
            pallas::codec::minicbor::encode(&x.transaction_body.inputs, &mut encoded_inputs)
                .unwrap();
            encoded_inputs
        }
        MultiEraTx::Babbage(x) => {
            let mut encoded_inputs: Vec<u8> = Vec::new();
            pallas::codec::minicbor::encode(&x.transaction_body.inputs, &mut encoded_inputs)
                .unwrap();
            encoded_inputs
        }
        MultiEraTx::Byron(x) => {
            let mut encoded_inputs: Vec<u8> = Vec::new();
            pallas::codec::minicbor::encode(&x.transaction.inputs, &mut encoded_inputs).unwrap();
            encoded_inputs
        }
        MultiEraTx::Conway(x) => {
            let mut encoded_inputs: Vec<u8> = Vec::new();
            pallas::codec::minicbor::encode(&x.transaction_body.inputs, &mut encoded_inputs)
                .unwrap();
            encoded_inputs
        }
        _ => Vec::new(),
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
