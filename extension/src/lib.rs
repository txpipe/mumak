mod era_ext;

use crate::era_ext::EraExt;

use bech32::{FromBase32, ToBase32};
use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};
use pallas::crypto::hash::Hasher;
use pallas::ledger::addresses::Address;
use pallas::ledger::addresses::ByronAddress;
use pallas::ledger::addresses::StakeAddress;
use pallas::ledger::primitives::ToCanonicalJson;
use pallas::ledger::traverse::wellknown::*;
use pallas::ledger::traverse::MultiEraBlock;
use pallas::ledger::traverse::MultiEraOutput;
use pallas::ledger::traverse::MultiEraTx;
use pallas::ledger::traverse::MultiEraWithdrawals;
use pgrx::prelude::*;
use std::collections::HashMap;
use std::ops::Deref;

pgrx::pg_module_magic!();

#[pg_extern]
fn hello_extension() -> &'static str {
    "Hello, extension"
}

#[pg_extern]
fn block_tx_count(block_cbor: &[u8]) -> i32 {
    let block = match MultiEraBlock::decode(block_cbor) {
        Ok(x) => x,
        Err(_) => return -1,
    };

    block.tx_count() as i32
}

#[pg_extern]
fn block_number(block_cbor: &[u8]) -> i64 {
    let block = match MultiEraBlock::decode(block_cbor) {
        Ok(x) => x,
        Err(_) => return -1,
    };

    block.number() as i64
}

#[pg_extern]
fn block_slot(block_cbor: &[u8]) -> i64 {
    let block = match MultiEraBlock::decode(block_cbor) {
        Ok(x) => x,
        Err(_) => return -1,
    };

    block.slot() as i64
}

#[pg_extern]
fn block_pool_id(block_cbor: &[u8]) -> Vec<u8> {
    let block = match MultiEraBlock::decode(block_cbor) {
        Ok(x) => x,
        Err(_) => return vec![],
    };
    match block.header().issuer_vkey() {
        Some(hash) => Hasher::<224>::hash(hash).to_vec(),
        None => vec![],
    }
}

#[pg_extern]
fn block_has_pool_id(block_cbor: &[u8], pool_id: &[u8]) -> bool {
    let block = match MultiEraBlock::decode(block_cbor) {
        Ok(x) => x,
        Err(_) => return false,
    };

    match block.header().issuer_vkey() {
        Some(hash) => Hasher::<224>::hash(hash).to_vec() == pool_id,
        None => false,
    }
}

#[pg_extern]
fn block_size(block_cbor: &[u8]) -> i64 {
    let block = match MultiEraBlock::decode(block_cbor) {
        Ok(x) => x,
        Err(_) => return -1,
    };

    block.size() as i64
}

#[pg_extern]
fn block_epoch(block_cbor: &[u8], network_id: i64) -> i64 {
    let block = match MultiEraBlock::decode(block_cbor) {
        Ok(x) => x,
        Err(_) => return -1,
    };

    let genesis = match GenesisValues::from_magic(network_id as u64) {
        Some(x) => x,
        None => return -1,
    };

    block.epoch(&genesis).0 as i64
}

#[pg_extern]
fn block_slot_as_time(block_cbor: &[u8], network_id: i64) -> pgrx::Timestamp {
    let block = match MultiEraBlock::decode(block_cbor) {
        Ok(x) => x,
        Err(_) => return (-1).into(),
    };

    let genesis = match GenesisValues::from_magic(network_id as u64) {
        Some(x) => x,
        None => return (-1).into(),
    };

    let seconds = block.wallclock(&genesis) as i64;

    let naive_datetime = DateTime::from_timestamp(seconds, 0)
        .unwrap_or_else(|| DateTime::from_timestamp(0, 0).unwrap());

    let year = naive_datetime.year();
    let month = naive_datetime.month() as u8;
    let day = naive_datetime.day() as u8;
    let hour = naive_datetime.hour() as u8;
    let minute = naive_datetime.minute() as u8;
    let second = naive_datetime.second() as f64;

    let timestamp = Timestamp::new(year, month, day, hour, minute, second).unwrap();

    timestamp
}

#[pg_extern]
fn block_is_epoch(block_cbor: &[u8], network_id: i64, epoch: i64) -> bool {
    let block = match MultiEraBlock::decode(block_cbor) {
        Ok(x) => x,
        Err(_) => return false,
    };

    let genesis = match GenesisValues::from_magic(network_id as u64) {
        Some(x) => x,
        None => return false,
    };

    block.epoch(&genesis).0 == epoch as u64
}

/// Returns the hash of the given transaction data.
///
/// # Arguments
///
/// * `tx_cbor` - The transaction data in CBOR format.
///
/// # Returns
///
/// The hash of the given transaction data as a string.
///
/// # Example
///
/// ```
/// select tx_hash(body) from transactions;
/// ```
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
fn tx_outputs(
    tx_cbor: &[u8],
) -> TableIterator<
    'static,
    (
        name!(output_index, i32),
        name!(address, Vec<u8>),
        name!(lovelace, pgrx::AnyNumeric),
        name!(assets, pgrx::Json),
        name!(datum, pgrx::Json),
    ),
> {
    let tx = match MultiEraTx::decode(tx_cbor) {
        Ok(x) => x,
        Err(_) => return TableIterator::new(std::iter::empty()),
    };

    let outputs_data = tx
        .outputs()
        .iter()
        .enumerate()
        .map(|(i, o)| {
            (
                i as i32,
                o.address().unwrap().to_vec(),
                AnyNumeric::from(o.lovelace_amount()),
                pgrx::Json(
                    serde_json::to_value(
                        o.non_ada_assets()
                            .iter()
                            .map(|asset| {
                                let policy_id = hex::encode(asset.policy().as_ref());
                                let assets: HashMap<String, i128> = asset
                                    .assets()
                                    .iter()
                                    .map(|a| (hex::encode(a.name()), a.any_coin()))
                                    .collect();

                                (policy_id, assets)
                            })
                            .collect::<HashMap<_, _>>(),
                    )
                    .unwrap(),
                ),
                match o.datum() {
                    Some(d) => match d {
                        pallas::ledger::primitives::conway::PseudoDatumOption::Hash(_) => {
                            pgrx::Json(serde_json::json!(null))
                        }
                        pallas::ledger::primitives::conway::PseudoDatumOption::Data(d) => {
                            pgrx::Json(d.unwrap().deref().to_json())
                        }
                    },
                    None => pgrx::Json(serde_json::json!(null)),
                },
            )
        })
        .collect::<Vec<_>>();

    TableIterator::new(outputs_data.into_iter())
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
fn tx_total_lovelace(tx_cbor: &[u8]) -> pgrx::AnyNumeric {
    let tx = match MultiEraTx::decode(tx_cbor) {
        Ok(x) => x,
        Err(_) => return AnyNumeric::from(0),
    };

    AnyNumeric::from(
        tx.outputs()
            .iter()
            .map(|o| o.lovelace_amount())
            .sum::<u64>(),
    )
}

#[pg_extern]
fn tx_fee(tx_cbor: &[u8]) -> pgrx::AnyNumeric {
    let tx = match MultiEraTx::decode(tx_cbor) {
        Ok(x) => x,
        Err(_) => return AnyNumeric::from(0),
    };
    let fee = match tx.fee() {
        Some(f) => f,
        None => return AnyNumeric::from(0),
    };
    AnyNumeric::from(fee)
}

#[pg_extern]
fn tx_mint(tx_cbor: &[u8]) -> pgrx::Json {
    let tx = match MultiEraTx::decode(tx_cbor) {
        Ok(x) => x,
        Err(_) => return pgrx::Json(serde_json::json!(null)),
    };

    let mints = tx.mints();

    pgrx::Json(
        serde_json::to_value(
            mints
                .iter()
                .map(|m| {
                    let policy_id = hex::encode(m.policy().as_ref());
                    let assets: HashMap<String, i128> = m
                        .assets()
                        .iter()
                        .map(|a| (hex::encode(a.name()), a.any_coin()))
                        .collect();

                    (policy_id, assets)
                })
                .collect::<HashMap<_, _>>(),
        )
        .unwrap(),
    )
}

#[pg_extern]
fn tx_subject_amount_output(tx_cbor: &[u8], subject: &[u8]) -> pgrx::AnyNumeric {
    let tx = match MultiEraTx::decode(tx_cbor) {
        Ok(x) => x,
        Err(_) => return AnyNumeric::from(0),
    };

    const POLICY_ID_LEN: usize = 28;
    let (policy_id, asset_name) = subject.split_at(POLICY_ID_LEN);

    let amount = tx
        .outputs()
        .iter()
        .filter(|o| {
            o.non_ada_assets().to_vec().iter().any(|a| {
                a.policy().deref() == policy_id && a.assets().iter().any(|a| a.name() == asset_name)
            })
        })
        .map(|o| {
            o.non_ada_assets()
                .to_vec()
                .iter()
                .flat_map(|a| {
                    a.assets()
                        .iter()
                        .filter(|a| a.name() == asset_name)
                        .map(|a| a.any_coin())
                        .collect::<Vec<_>>()
                })
                .next()
                .unwrap_or(0)
        })
        .sum::<i128>();

    AnyNumeric::from(amount)
}

#[pg_extern]
fn tx_subject_amount_mint(tx_cbor: &[u8], subject: &[u8]) -> pgrx::AnyNumeric {
    let tx = match MultiEraTx::decode(tx_cbor) {
        Ok(x) => x,
        Err(_) => return AnyNumeric::from(0),
    };

    const POLICY_ID_LEN: usize = 28;
    let (policy_id, asset_name) = subject.split_at(POLICY_ID_LEN);

    let amount = tx
        .mints()
        .iter()
        .filter(|m| {
            m.assets().iter().any(|a| a.policy().deref() == policy_id && a.name() == asset_name)
        })
        .map(|m| {
            m.assets()
                .iter()
                .filter(|a| a.name() == asset_name)
                .map(|a| a.any_coin())
                .sum::<i128>()
        })
        .sum::<i128>();

    AnyNumeric::from(amount)
}

#[pg_extern]
fn tx_withdrawals(tx_cbor: &[u8]) -> TableIterator<'static, (name!(stake_address, Vec<u8>), name!(amount, pgrx::AnyNumeric))> {
    let tx = match MultiEraTx::decode(tx_cbor) {
        Ok(x) => x,
        Err(_) => return TableIterator::new(std::iter::empty()),
    };

    let withdrawals_data = match tx.withdrawals() {
        MultiEraWithdrawals::AlonzoCompatible(w) => w.iter().map(|(k, v)| (k.to_vec(), AnyNumeric::from(*v))).collect::<Vec<_>>(),
        _ => vec![],
    };
    TableIterator::new(withdrawals_data.into_iter())
}

#[pg_extern]
fn tx_hash_is(tx_cbor: &[u8], hash: &[u8]) -> bool {
    let tx = match MultiEraTx::decode(tx_cbor) {
        Ok(x) => x,
        Err(_) => return false,
    };

    tx.hash().to_vec().eq(&hash)
}

#[pg_extern]
fn tx_has_mint(tx_cbor: &[u8]) -> bool {
    let tx = match MultiEraTx::decode(tx_cbor) {
        Ok(x) => x,
        Err(_) => return false,
    };

    !tx.mints().is_empty()
}

#[pg_extern]
fn tx_has_address_output(tx_cbor: &[u8], address: &[u8]) -> bool {
    let tx = match MultiEraTx::decode(tx_cbor) {
        Ok(x) => x,
        Err(_) => return false,
    };

    tx.outputs()
        .iter()
        .any(|o| o.address().unwrap().to_vec().eq(&address))
}

#[pg_extern]
fn tx_has_policy_id_output(tx_cbor: &[u8], policy_id: &[u8]) -> bool {
    let tx = match MultiEraTx::decode(tx_cbor) {
        Ok(x) => x,
        Err(_) => return false,
    };

    tx.outputs().iter().any(|o| {
        o.non_ada_assets()
            .to_vec()
            .iter()
            .any(|a| a.policy().deref().eq(&policy_id))
    })
}

#[pg_extern]
fn tx_has_policy_id_mint(tx_cbor: &[u8], policy_id: &[u8]) -> bool {
    let tx = match MultiEraTx::decode(tx_cbor) {
        Ok(x) => x,
        Err(_) => return false,
    };

    tx.mints().iter().any(|m| m.policy().deref().eq(&policy_id))
}

#[pg_extern]
fn tx_has_subject_output(tx_cbor: &[u8], subject: &[u8]) -> bool {
    let tx = match MultiEraTx::decode(tx_cbor) {
        Ok(x) => x,
        Err(_) => return false,
    };

    const POLICY_ID_LEN: usize = 28;
    let (policy_id, asset_name) = subject.split_at(POLICY_ID_LEN);

    tx.outputs().iter().any(|o| {
        o.non_ada_assets().to_vec().iter().any(|a| {
            a.policy().deref() == policy_id && a.assets().iter().any(|a| a.name() == asset_name)
        })
    })
}

#[pg_extern]
fn tx_has_mint_output(tx_cbor: &[u8], subject: &[u8]) -> bool {
    let tx = match MultiEraTx::decode(tx_cbor) {
        Ok(x) => x,
        Err(_) => return false,
    };

    const POLICY_ID_LEN: usize = 28;
    let (policy_id, asset_name) = subject.split_at(POLICY_ID_LEN);

    tx.mints().iter().any(|m| {
        m.assets().iter().any(|a| a.policy().deref() == policy_id && a.name() == asset_name)
    })
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

#[pg_extern]
fn address_to_bytes(address: String) -> Vec<u8> {
    let address = match Address::from_bech32(&address) {
        Ok(x) => x,
        Err(_) => return vec![],
    };

    address.to_vec()
}

#[pg_extern]
fn address_to_bech32(address_bytes: &[u8]) -> String {
    let address = match Address::from_bytes(address_bytes) {
        Ok(x) => x,
        Err(_) => return String::new(),
    };

    match address.to_bech32() {
        Ok(x) => x,
        // @TODO: this is not bech32 though?
        Err(_) => ByronAddress::from_bytes(address_bytes).unwrap().to_base58(),
    }
}

#[pg_extern]
fn address_to_stake_part_bech32(address_bytes: &[u8]) -> String {
    let address = match Address::from_bytes(address_bytes) {
        Ok(addr) => addr,
        Err(_) => return String::new(),
    };

    match address {
        Address::Shelley(a) => StakeAddress::try_from(a)
            .map(|stake_addr| stake_addr.to_bech32().unwrap_or_else(|_| String::new()))
            .unwrap_or_else(|_| String::new()),
        Address::Byron(_) => String::new(),
        _ => String::new(),
    }
}

#[pg_extern]
fn stake_part_to_bech32(stake_part_bytes: &[u8]) -> String {
    let stake_part = match Address::from_bytes(stake_part_bytes) {
        Ok(x) => x,
        Err(_) => return String::new(),
    };

    match stake_part.to_bech32() {
        Ok(x) => x,
        Err(_) => String::new(),
    }
}

#[pg_extern]
fn utxo_address(era: i32, utxo_cbor: &[u8]) -> Vec<u8> {
    let era_enum = match pallas::ledger::traverse::Era::from_int(era) {
        Some(x) => x,
        None => return vec![],
    };

    let output = match MultiEraOutput::decode(era_enum, utxo_cbor) {
        Ok(x) => x,
        Err(_) => return vec![],
    };

    output.address().unwrap().to_vec()
}

#[pg_extern]
fn utxo_has_policy_id(era: i32, utxo_cbor: &[u8], policy_id: &[u8]) -> bool {
    let era_enum = match pallas::ledger::traverse::Era::from_int(era) {
        Some(x) => x,
        None => return false,
    };

    let output = match MultiEraOutput::decode(era_enum, utxo_cbor) {
        Ok(x) => x,
        Err(_) => return false,
    };

    output
        .non_ada_assets()
        .to_vec()
        .iter()
        .any(|a| a.policy().deref().eq(&policy_id))
}

#[pg_extern]
fn utxo_has_address(era: i32, utxo_cbor: &[u8], address: &[u8]) -> bool {
    let era_enum = match pallas::ledger::traverse::Era::from_int(era) {
        Some(x) => x,
        None => return false,
    };

    let output = match MultiEraOutput::decode(era_enum, utxo_cbor) {
        Ok(x) => x,
        Err(_) => return false,
    };

    output.address().unwrap().to_vec().eq(&address)
}

#[pg_extern]
fn utxo_lovelace(era: i32, utxo_cbor: &[u8]) -> pgrx::AnyNumeric {
    let era_enum = match pallas::ledger::traverse::Era::from_int(era) {
        Some(x) => x,
        None => return AnyNumeric::from(0),
    };

    let output = match MultiEraOutput::decode(era_enum, utxo_cbor) {
        Ok(x) => x,
        Err(_) => return AnyNumeric::from(0),
    };

    AnyNumeric::from(output.lovelace_amount())
}

#[pg_extern]
fn utxo_policy_id_asset_names(
    era: i32,
    utxo_cbor: &[u8],
    policy_id: &[u8],
) -> SetOfIterator<'static, Vec<u8>> {
    let era_enum = match pallas::ledger::traverse::Era::from_int(era) {
        Some(x) => x,
        None => return SetOfIterator::new(std::iter::empty()),
    };

    let output = match MultiEraOutput::decode(era_enum, utxo_cbor) {
        Ok(x) => x,
        Err(_) => return SetOfIterator::new(std::iter::empty()),
    };

    let asset_names = output
        .non_ada_assets()
        .to_vec()
        .iter()
        .filter(|a| a.policy().deref().eq(&policy_id))
        .flat_map(|a| {
            a.assets()
                .iter()
                .map(|a| a.name().to_vec())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    SetOfIterator::new(asset_names)
}

#[pg_extern]
fn utxo_asset_values(
    era: i32,
    utxo_cbor: &[u8],
) -> TableIterator<
    'static,
    (
        name!(policy_id, Vec<u8>),
        name!(asset_name, Vec<u8>),
        name!(amount, pgrx::AnyNumeric),
    ),
> {
    let era_enum = match pallas::ledger::traverse::Era::from_int(era) {
        Some(x) => x,
        None => return TableIterator::new(std::iter::empty()),
    };

    let output = match MultiEraOutput::decode(era_enum, utxo_cbor) {
        Ok(x) => x,
        Err(_) => return TableIterator::new(std::iter::empty()),
    };

    let asset_values = output
        .non_ada_assets()
        .to_vec()
        .iter()
        .flat_map(|a| {
            a.assets()
                .iter()
                .map(|a| {
                    (
                        a.policy().to_vec(),
                        a.name().to_vec(),
                        AnyNumeric::from(a.any_coin()),
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    TableIterator::new(asset_values.into_iter())
}

#[pg_extern]
fn utxo_policy_id_asset_values(
    era: i32,
    utxo_cbor: &[u8],
    policy_id: &[u8],
) -> TableIterator<'static, (name!(asset_name, Vec<u8>), name!(amount, pgrx::AnyNumeric))> {
    let era_enum = match pallas::ledger::traverse::Era::from_int(era) {
        Some(x) => x,
        None => return TableIterator::new(std::iter::empty()),
    };

    let output = match MultiEraOutput::decode(era_enum, utxo_cbor) {
        Ok(x) => x,
        Err(_) => return TableIterator::new(std::iter::empty()),
    };

    let asset_values = output
        .non_ada_assets()
        .to_vec()
        .iter()
        .filter(|a| a.policy().deref().eq(&policy_id))
        .flat_map(|a| {
            a.assets()
                .iter()
                .map(|a| (a.name().to_vec(), AnyNumeric::from(a.any_coin())))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    TableIterator::new(asset_values.into_iter())
}

#[pg_extern]
fn utxo_subject_amount(era: i32, utxo_cbor: &[u8], subject: &[u8]) -> pgrx::AnyNumeric {
    let era_enum = match pallas::ledger::traverse::Era::from_int(era) {
        Some(x) => x,
        None => return AnyNumeric::from(0),
    };

    let output = match MultiEraOutput::decode(era_enum, utxo_cbor) {
        Ok(x) => x,
        Err(_) => return AnyNumeric::from(0),
    };

    const POLICY_ID_LEN: usize = 28;
    let (policy_id, asset_name) = subject.split_at(POLICY_ID_LEN);

    let amount = output
        .non_ada_assets()
        .iter()
        .filter(|a| a.policy().deref() == policy_id)
        .flat_map(|a| {
            a.assets()
                .iter()
                .filter(|a| a.name() == asset_name)
                .map(|a| a.any_coin())
                .collect::<Vec<_>>()
        })
        .next()
        .unwrap_or(0);

    AnyNumeric::from(amount)
}

#[pg_extern]
fn utxo_plutus_data(era: i32, utxo_cbor: &[u8]) -> pgrx::Json {
    let era_enum = match pallas::ledger::traverse::Era::from_int(era) {
        Some(x) => x,
        None => return pgrx::Json(serde_json::json!(null)),
    };

    let output = match MultiEraOutput::decode(era_enum, utxo_cbor) {
        Ok(x) => x,
        Err(_) => return pgrx::Json(serde_json::json!(null)),
    };

    match output.datum().unwrap() {
        pallas::ledger::primitives::conway::PseudoDatumOption::Hash(_) => {
            pgrx::Json(serde_json::json!(null))
        }
        pallas::ledger::primitives::conway::PseudoDatumOption::Data(d) => {
            pgrx::Json(d.unwrap().deref().to_json())
        }
    }
}

#[pg_extern]
fn to_bech32(hash: &[u8], hrp: &str) -> String {
    match bech32::encode(hrp, &hash.to_base32(), bech32::Variant::Bech32) {
        Ok(x) => x,
        Err(_) => "".to_string(),
    }
}

#[pg_extern]
fn from_bech32(bech32: &str) -> Vec<u8> {
    match bech32::decode(bech32) {
        Ok((_, data, _)) => Vec::from_base32(&data).unwrap(),
        Err(_) => vec![],
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

    #[pg_test]
    fn test_tx_hash() {
        // Decoded transaction data for testing
        const TX_DATA_HEX: &str = "84ad009282582040e50ebf0ded25391f7dd13ad2d32a8eef5a2cc76cc0e95b8bb2330c482def2f0082582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520082582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520282582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520382582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520482582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520582582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520682582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520782582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520882582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520982582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520a82582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520b82582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520c82582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520d82582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520e82582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520f82582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822521082582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252110182a300581d71071bd7f4b5e059ea90e763467cf559167b21c82ef1cb5fe34fb7a9e501821a030a32c0a3581c1cc1aceaf5c7df55e270864a60600b9f52383fe418164574ffdeeed0a14010581cc0e5564cf5786031d9053f567ec78b8383a0f2bc01318e690e0503f4a14001581cf66d78b4a3cb3d37afa0ec36461e51ecbde00f26c8f0a68f94b69880a144695553441b00000201d16e7cf2028201d818479f0000000000ff82583901da299558c70a8970781806dca93d1801ba2f3b3894227a7b284786e49baba19195b7cb8b1c6febb192cc487b5e8b96d737baddb8bb09866f1b000000015053786b021a0007a272031a07138899075820d36a2619a672494604e11bb447cbcf5231e9f2ba25c2169177edc941bd50ad6c081a0713876d0b5820de92cfe211abe2b770d253ff364362e4281c96ce70c3048b104acb5fc172ea900d8182582040e50ebf0ded25391f7dd13ad2d32a8eef5a2cc76cc0e95b8bb2330c482def2f000e81581cda299558c70a8970781806dca93d1801ba2f3b3894227a7b284786e40f011082583901da299558c70a8970781806dca93d1801ba2f3b3894227a7b284786e49baba19195b7cb8b1c6febb192cc487b5e8b96d737baddb8bb09866f1b00000001502d541d111a002dc6c0128482582032536acbfa12b80a3c570b1dac7948187dfa66992460d11542f67ba357c0fd2c0082582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520182582089f6715ff7affd8bdeff696f47d7a08bd899cc9c627483a8885f9fd3943286a100825820db7900797bf9c1235976b226d0cdbe1040d199555158bfe2bc042575f142da6100a30082825820f44ce6186d190f8776fd871d753df7ae503972e4793a2360a423d2f96021e60158400b18e4fcf4be17a531d3fd7a0320df6ba7acbff31b35118275d7ff1cb3de25523453c767af39ac1f3b435749c44af64ecbaf19ae1e23a7b4ab9c7939d653a90182582063179f731829d60aade12a1398c07b7a905cc38e7d9901850c9b186946f5ca3e58403b3932c709d9a355f8a0bb453d2722f39f82a16bb7669669f11698cacc825ce2a74b26aa31f3740a8a820829bfda3f6f3f4bbce1f045707d037df0085273a50004800591840001d87980821a001aaf3e1a315977a684000ad87980821a00011efa1a01c4794f840006d87980821a00011efa1a01c4794f840004d87980821a00011efa1a01c4794f840002d87980821a00011efa1a01c4794f840003d87980821a00011efa1a01c4794f840005d87980821a00011efa1a01c4794f840008d87980821a00011efa1a01c4794f840007d87980821a00011efa1a01c4794f840009d87980821a00011efa1a01c4794f84000ed87980821a00011efa1a01c4794f84000cd87980821a00011efa1a01c4794f84000bd87980821a00011efa1a01c4794f84000dd87980821a00011efa1a01c4794f840010d87980821a00011efa1a01c4794f84000fd87980821a00011efa1a01c4794f840011d87980821a00011efa1a01c4794ff5a0";
        // Expected hash result for the given transaction data
        const EXPECTED_HASH: &str =
            "691bb954d364ac5a2fe4bafc72b43a77edee54bd4237d748547426f14f304c96";

        let tx_cbor = hex::decode(TX_DATA_HEX).expect("Failed to decode hex string into bytes");

        assert_eq!(
            EXPECTED_HASH,
            crate::tx_hash(&tx_cbor).to_string(),
            "The hash of the provided transaction data did not match the expected value."
        );
    }

    #[pg_test]
    fn test_tx_inputs() {
        // Decoded transaction data for testing
        const TX_DATA_HEX: &str = "84ad009282582040e50ebf0ded25391f7dd13ad2d32a8eef5a2cc76cc0e95b8bb2330c482def2f0082582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520082582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520282582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520382582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520482582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520582582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520682582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520782582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520882582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520982582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520a82582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520b82582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520c82582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520d82582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520e82582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520f82582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822521082582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252110182a300581d71071bd7f4b5e059ea90e763467cf559167b21c82ef1cb5fe34fb7a9e501821a030a32c0a3581c1cc1aceaf5c7df55e270864a60600b9f52383fe418164574ffdeeed0a14010581cc0e5564cf5786031d9053f567ec78b8383a0f2bc01318e690e0503f4a14001581cf66d78b4a3cb3d37afa0ec36461e51ecbde00f26c8f0a68f94b69880a144695553441b00000201d16e7cf2028201d818479f0000000000ff82583901da299558c70a8970781806dca93d1801ba2f3b3894227a7b284786e49baba19195b7cb8b1c6febb192cc487b5e8b96d737baddb8bb09866f1b000000015053786b021a0007a272031a07138899075820d36a2619a672494604e11bb447cbcf5231e9f2ba25c2169177edc941bd50ad6c081a0713876d0b5820de92cfe211abe2b770d253ff364362e4281c96ce70c3048b104acb5fc172ea900d8182582040e50ebf0ded25391f7dd13ad2d32a8eef5a2cc76cc0e95b8bb2330c482def2f000e81581cda299558c70a8970781806dca93d1801ba2f3b3894227a7b284786e40f011082583901da299558c70a8970781806dca93d1801ba2f3b3894227a7b284786e49baba19195b7cb8b1c6febb192cc487b5e8b96d737baddb8bb09866f1b00000001502d541d111a002dc6c0128482582032536acbfa12b80a3c570b1dac7948187dfa66992460d11542f67ba357c0fd2c0082582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520182582089f6715ff7affd8bdeff696f47d7a08bd899cc9c627483a8885f9fd3943286a100825820db7900797bf9c1235976b226d0cdbe1040d199555158bfe2bc042575f142da6100a30082825820f44ce6186d190f8776fd871d753df7ae503972e4793a2360a423d2f96021e60158400b18e4fcf4be17a531d3fd7a0320df6ba7acbff31b35118275d7ff1cb3de25523453c767af39ac1f3b435749c44af64ecbaf19ae1e23a7b4ab9c7939d653a90182582063179f731829d60aade12a1398c07b7a905cc38e7d9901850c9b186946f5ca3e58403b3932c709d9a355f8a0bb453d2722f39f82a16bb7669669f11698cacc825ce2a74b26aa31f3740a8a820829bfda3f6f3f4bbce1f045707d037df0085273a50004800591840001d87980821a001aaf3e1a315977a684000ad87980821a00011efa1a01c4794f840006d87980821a00011efa1a01c4794f840004d87980821a00011efa1a01c4794f840002d87980821a00011efa1a01c4794f840003d87980821a00011efa1a01c4794f840005d87980821a00011efa1a01c4794f840008d87980821a00011efa1a01c4794f840007d87980821a00011efa1a01c4794f840009d87980821a00011efa1a01c4794f84000ed87980821a00011efa1a01c4794f84000cd87980821a00011efa1a01c4794f84000bd87980821a00011efa1a01c4794f84000dd87980821a00011efa1a01c4794f840010d87980821a00011efa1a01c4794f84000fd87980821a00011efa1a01c4794f840011d87980821a00011efa1a01c4794ff5a0";
        // Expected inputs for the given transaction data
        const EXPECTED_INPUTS: &[(&str, i64)] = &[
            (
                "40e50ebf0ded25391f7dd13ad2d32a8eef5a2cc76cc0e95b8bb2330c482def2f",
                0,
            ),
            (
                "83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",
                0,
            ),
            (
                "83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",
                2,
            ),
            (
                "83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",
                3,
            ),
            (
                "83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",
                4,
            ),
            (
                "83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",
                5,
            ),
            (
                "83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",
                6,
            ),
            (
                "83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",
                7,
            ),
            (
                "83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",
                8,
            ),
            (
                "83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",
                9,
            ),
            (
                "83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",
                10,
            ),
            (
                "83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",
                11,
            ),
            (
                "83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",
                12,
            ),
            (
                "83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",
                13,
            ),
            (
                "83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",
                14,
            ),
            (
                "83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",
                15,
            ),
            (
                "83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",
                16,
            ),
            (
                "83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",
                17,
            ),
        ];

        let tx_cbor = hex::decode(TX_DATA_HEX).expect("Failed to decode hex string into bytes");

        let inputs = crate::tx_inputs(&tx_cbor).collect::<Vec<_>>();

        let expected_inputs = EXPECTED_INPUTS
            .iter()
            .map(|&(hex, index)| (String::from(hex), index))
            .collect::<Vec<_>>();

        assert_eq!(
            expected_inputs, inputs,
            "The transaction inputs did not match the expected values."
        );
    }

    #[pg_test]
    fn test_tx_inputs_json() {
        const TX_DATA_HEX: &str = "84ad009282582040e50ebf0ded25391f7dd13ad2d32a8eef5a2cc76cc0e95b8bb2330c482def2f0082582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520082582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520282582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520382582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520482582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520582582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520682582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520782582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520882582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520982582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520a82582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520b82582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520c82582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520d82582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520e82582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520f82582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822521082582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252110182a300581d71071bd7f4b5e059ea90e763467cf559167b21c82ef1cb5fe34fb7a9e501821a030a32c0a3581c1cc1aceaf5c7df55e270864a60600b9f52383fe418164574ffdeeed0a14010581cc0e5564cf5786031d9053f567ec78b8383a0f2bc01318e690e0503f4a14001581cf66d78b4a3cb3d37afa0ec36461e51ecbde00f26c8f0a68f94b69880a144695553441b00000201d16e7cf2028201d818479f0000000000ff82583901da299558c70a8970781806dca93d1801ba2f3b3894227a7b284786e49baba19195b7cb8b1c6febb192cc487b5e8b96d737baddb8bb09866f1b000000015053786b021a0007a272031a07138899075820d36a2619a672494604e11bb447cbcf5231e9f2ba25c2169177edc941bd50ad6c081a0713876d0b5820de92cfe211abe2b770d253ff364362e4281c96ce70c3048b104acb5fc172ea900d8182582040e50ebf0ded25391f7dd13ad2d32a8eef5a2cc76cc0e95b8bb2330c482def2f000e81581cda299558c70a8970781806dca93d1801ba2f3b3894227a7b284786e40f011082583901da299558c70a8970781806dca93d1801ba2f3b3894227a7b284786e49baba19195b7cb8b1c6febb192cc487b5e8b96d737baddb8bb09866f1b00000001502d541d111a002dc6c0128482582032536acbfa12b80a3c570b1dac7948187dfa66992460d11542f67ba357c0fd2c0082582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520182582089f6715ff7affd8bdeff696f47d7a08bd899cc9c627483a8885f9fd3943286a100825820db7900797bf9c1235976b226d0cdbe1040d199555158bfe2bc042575f142da6100a30082825820f44ce6186d190f8776fd871d753df7ae503972e4793a2360a423d2f96021e60158400b18e4fcf4be17a531d3fd7a0320df6ba7acbff31b35118275d7ff1cb3de25523453c767af39ac1f3b435749c44af64ecbaf19ae1e23a7b4ab9c7939d653a90182582063179f731829d60aade12a1398c07b7a905cc38e7d9901850c9b186946f5ca3e58403b3932c709d9a355f8a0bb453d2722f39f82a16bb7669669f11698cacc825ce2a74b26aa31f3740a8a820829bfda3f6f3f4bbce1f045707d037df0085273a50004800591840001d87980821a001aaf3e1a315977a684000ad87980821a00011efa1a01c4794f840006d87980821a00011efa1a01c4794f840004d87980821a00011efa1a01c4794f840002d87980821a00011efa1a01c4794f840003d87980821a00011efa1a01c4794f840005d87980821a00011efa1a01c4794f840008d87980821a00011efa1a01c4794f840007d87980821a00011efa1a01c4794f840009d87980821a00011efa1a01c4794f84000ed87980821a00011efa1a01c4794f84000cd87980821a00011efa1a01c4794f84000bd87980821a00011efa1a01c4794f84000dd87980821a00011efa1a01c4794f840010d87980821a00011efa1a01c4794f84000fd87980821a00011efa1a01c4794f840011d87980821a00011efa1a01c4794ff5a0";
        const EXPECTED_JSON: &str = r#"[["40e50ebf0ded25391f7dd13ad2d32a8eef5a2cc76cc0e95b8bb2330c482def2f",0],["83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",0],["83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",2],["83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",3],["83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",4],["83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",5],["83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",6],["83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",7],["83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",8],["83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",9],["83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",10],["83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",11],["83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",12],["83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",13],["83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",14],["83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",15],["83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",16],["83d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252",17]]"#;

        let tx_cbor = hex::decode(TX_DATA_HEX).expect("Failed to decode hex string into bytes");

        let result_json = crate::tx_inputs_json(&tx_cbor);

        let result_data: Vec<(String, u64)> =
            serde_json::from_value(result_json.0).expect("Failed to deserialize JSON result");

        let expected_data: Vec<(String, u64)> =
            serde_json::from_str(EXPECTED_JSON).expect("Failed to deserialize expected JSON");

        assert_eq!(
            expected_data, result_data,
            "The JSON transaction inputs did not match the expected values."
        );
    }

    #[pg_test]
    fn test_tx_inputs_cbor() {
        const TX_DATA_HEX: &str = "84ad009282582040e50ebf0ded25391f7dd13ad2d32a8eef5a2cc76cc0e95b8bb2330c482def2f0082582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520082582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520282582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520382582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520482582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520582582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520682582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520782582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520882582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520982582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520a82582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520b82582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520c82582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520d82582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520e82582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520f82582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822521082582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f282252110182a300581d71071bd7f4b5e059ea90e763467cf559167b21c82ef1cb5fe34fb7a9e501821a030a32c0a3581c1cc1aceaf5c7df55e270864a60600b9f52383fe418164574ffdeeed0a14010581cc0e5564cf5786031d9053f567ec78b8383a0f2bc01318e690e0503f4a14001581cf66d78b4a3cb3d37afa0ec36461e51ecbde00f26c8f0a68f94b69880a144695553441b00000201d16e7cf2028201d818479f0000000000ff82583901da299558c70a8970781806dca93d1801ba2f3b3894227a7b284786e49baba19195b7cb8b1c6febb192cc487b5e8b96d737baddb8bb09866f1b000000015053786b021a0007a272031a07138899075820d36a2619a672494604e11bb447cbcf5231e9f2ba25c2169177edc941bd50ad6c081a0713876d0b5820de92cfe211abe2b770d253ff364362e4281c96ce70c3048b104acb5fc172ea900d8182582040e50ebf0ded25391f7dd13ad2d32a8eef5a2cc76cc0e95b8bb2330c482def2f000e81581cda299558c70a8970781806dca93d1801ba2f3b3894227a7b284786e40f011082583901da299558c70a8970781806dca93d1801ba2f3b3894227a7b284786e49baba19195b7cb8b1c6febb192cc487b5e8b96d737baddb8bb09866f1b00000001502d541d111a002dc6c0128482582032536acbfa12b80a3c570b1dac7948187dfa66992460d11542f67ba357c0fd2c0082582083d52903a465b2cf0dbb0900c1d8a1e2dec10578075cd7484b869a205f2822520182582089f6715ff7affd8bdeff696f47d7a08bd899cc9c627483a8885f9fd3943286a100825820db7900797bf9c1235976b226d0cdbe1040d199555158bfe2bc042575f142da6100a30082825820f44ce6186d190f8776fd871d753df7ae503972e4793a2360a423d2f96021e60158400b18e4fcf4be17a531d3fd7a0320df6ba7acbff31b35118275d7ff1cb3de25523453c767af39ac1f3b435749c44af64ecbaf19ae1e23a7b4ab9c7939d653a90182582063179f731829d60aade12a1398c07b7a905cc38e7d9901850c9b186946f5ca3e58403b3932c709d9a355f8a0bb453d2722f39f82a16bb7669669f11698cacc825ce2a74b26aa31f3740a8a820829bfda3f6f3f4bbce1f045707d037df0085273a50004800591840001d87980821a001aaf3e1a315977a684000ad87980821a00011efa1a01c4794f840006d87980821a00011efa1a01c4794f840004d87980821a00011efa1a01c4794f840002d87980821a00011efa1a01c4794f840003d87980821a00011efa1a01c4794f840005d87980821a00011efa1a01c4794f840008d87980821a00011efa1a01c4794f840007d87980821a00011efa1a01c4794f840009d87980821a00011efa1a01c4794f84000ed87980821a00011efa1a01c4794f84000cd87980821a00011efa1a01c4794f84000bd87980821a00011efa1a01c4794f84000dd87980821a00011efa1a01c4794f840010d87980821a00011efa1a01c4794f84000fd87980821a00011efa1a01c4794f840011d87980821a00011efa1a01c4794ff5a0";

        const EXPECTED_CBOR_HEX: &str = "928278403430653530656266306465643235333931663764643133616432643332613865656635613263633736636330653935623862623233333063343832646566326600827840383364353239303361343635623263663064626230393030633164386131653264656331303537383037356364373438346238363961323035663238323235320082784038336435323930336134363562326366306462623039303063316438613165326465633130353738303735636437343834623836396132303566323832323532028278403833643532393033613436356232636630646262303930306331643861316532646563313035373830373563643734383462383639613230356632383232353203827840383364353239303361343635623263663064626230393030633164386131653264656331303537383037356364373438346238363961323035663238323235320482784038336435323930336134363562326366306462623039303063316438613165326465633130353738303735636437343834623836396132303566323832323532058278403833643532393033613436356232636630646262303930306331643861316532646563313035373830373563643734383462383639613230356632383232353206827840383364353239303361343635623263663064626230393030633164386131653264656331303537383037356364373438346238363961323035663238323235320782784038336435323930336134363562326366306462623039303063316438613165326465633130353738303735636437343834623836396132303566323832323532088278403833643532393033613436356232636630646262303930306331643861316532646563313035373830373563643734383462383639613230356632383232353209827840383364353239303361343635623263663064626230393030633164386131653264656331303537383037356364373438346238363961323035663238323235320a827840383364353239303361343635623263663064626230393030633164386131653264656331303537383037356364373438346238363961323035663238323235320b827840383364353239303361343635623263663064626230393030633164386131653264656331303537383037356364373438346238363961323035663238323235320c827840383364353239303361343635623263663064626230393030633164386131653264656331303537383037356364373438346238363961323035663238323235320d827840383364353239303361343635623263663064626230393030633164386131653264656331303537383037356364373438346238363961323035663238323235320e827840383364353239303361343635623263663064626230393030633164386131653264656331303537383037356364373438346238363961323035663238323235320f82784038336435323930336134363562326366306462623039303063316438613165326465633130353738303735636437343834623836396132303566323832323532108278403833643532393033613436356232636630646262303930306331643861316532646563313035373830373563643734383462383639613230356632383232353211";

        let tx_cbor = hex::decode(TX_DATA_HEX).expect("Failed to decode hex string into bytes");

        let expected_cbor_bytes = hex::decode(EXPECTED_CBOR_HEX)
            .expect("Failed to decode expected CBOR hex string into bytes");

        let result_cbor_bytes = crate::tx_inputs_cbor(&tx_cbor);

        assert_eq!(
            expected_cbor_bytes, result_cbor_bytes,
            "The CBOR-encoded transaction inputs did not match the expected values."
        );
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
