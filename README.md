# Mumak

A custom PostgreSQL extension to interact with Cardano CBOR data directly.

## Introduction

The defacto wire format used to exchange Cardano data is CBOR. By Cardano data we refer to Blocks, Transactions, UTxOs, Certificates and all of its inner structures.

When indexing this data onto a relational database, such as PostgresQL, we usually deserialize the CBOR structures and map the values to different columns & tables in our schema.

Some times having a normalized data model is exactly what your use-case needs, but there are multiple scenarios where this mapping just adds complexity and performance penalties.

Mumak is a PostgreSQL extension that provides several custom functions to interact with Cardano CBOR data directly on the database.

For example, this means that you could store a whole Cardano Tx as CBOR in a bytes PostgreSQL column and then use SQL to ask things such as:

- Does this transaction involve address X?
- What's the total ADA output of this transaction?
- Is this transaction minting a token for policy X?

And the evaluation of this values will happen in-process, as part of the Db query execution.

## Available Functions

This is a list of the available / planned functions that are / will be supported by Mumak.

### `tx_lovelace_output`
Evaluates the total lovelace output of the transaction.

`tx_lovelace_output(tx_cbor, tx_era)`

### `tx_asset_output`

Evaluates the total output of a specific asset of the transaction.

`tx_asset_output(tx_cbor, tx_era, policy_id, asset_name)`

### `tx_has_address`
Is a predicate that evaluates tx cbor structures of the specified era. It will output true if the 

`tx_has_address(tx_cbor, tx_era, address_bech32)`

### `tx_hash`
Computes the hash of a transaction from its CBOR representation

`tx_hash(tx_cbor, tx_era)`

### `tx_has_input`
Is a predicate that evaluates if tx cbor structure of the specified era contains a particular input.

`tx_has_address(tx_cbor, tx_era, input_tx_hash, input_output_index)`

### `tx_is_valid`
Is a predicate that evaluates if tx cbor structure is considered a valid transaction from the perspective of the node validation.

`tx_is_valid(tx_cbor, tx_era)`

### `tx_has_asset`
A predicate that evaluates if tx cbor structure shows an output containing a particular asset.

`tx_has_asset(tx_cbor, tx_era, policy_id, asset_name)`

### `tx_asset_mint`
Evaluates the total assets of a specified of a specified policy / name that are minted by the transaction.

`tx_asset_mint(tx_cbor, tx_era, policy_id, asset_name)`

### `tx_asset_burn`
Evaluates the total assets of a specified of a specified policy / name that are burned by the transaction.

`tx_asset_mint(tx_cbor, tx_era, policy_id, asset_name)`

### `tx_has_datum`
A predicate that evaluates if tx cbor structure shows an output containing a particular datum.

`tx_has_datum(tx_cbor, tx_era, datum_hash)`

### `tx_has_metadata_label`
A predicate that evaluates if the tx cbor structure includes metadata with a particular label
	
`tx_has_metadata_label(tx_cbor, tx_era, label)`
	
### `tx_metadata_label`
Evaluates the metadatum value for a particular label contained in the tx cbor structure.

## Getting Started

// TODO

## Ecosystem

### Oura Integration [PLANNED]

An Oura sink will be developed to output common Cardano structures (Blocks, Transactions, UTxOs, Certificates) to a well-known schema that leverages the Mumak extensions for further querying.

Using Oura's integration with Mithril, this provides a way to bootstrap a queryable Cardano dataset in very little time.

### Scrolls Integration [PLANNED]

Scrolls will provide helper utility functions to create reducers that map data into Mumak-compatible records that can be stored in PostgreSQL directly. This approach simplifies the complexity of building complex reducer algorithms by offloading some of the complexity to the PostgreSQL database.

### Demeter Integration [PLANNED]

Demeter will provide a highly available, horiztonally scalable access to Mumak data as part of their services. Developers will be able to access a PostgreSQL instance directly and perform SQL queries on common Cardano CBOR structures using Mumak statements.

Data will be updated continously showing the most recent data from each of the common network in the Cardano blockchain ecosystem.
