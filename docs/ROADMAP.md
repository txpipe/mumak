### tx_hash
Computes the hash of a transaction from its CBOR representation

`tx_hash(tx_cbor) -> String`

### tx_inputs

Returns the list of inputs of the transaction.

`tx_inputs(tx_cbor) -> Json`

### tx_outputs

Returns the list of outputs of the transaction.

`tx_outputs(tx_cbor) -> Json`

### tx_lovelace_output
Evaluates the total lovelace output of the transaction.

`tx_lovelace_output(tx_cbor) -> Int`


### tx_lovelace_fee

Evaluates the total lovelace fee of the transaction.

`tx_lovelace_fee(tx_cbor) -> Int`

### tx_mints

Returns the list of minted tokens of the transaction.

`tx_mints(tx_cbor) -> Json`

### tx_validity_interval

Returns the validity interval of the transaction.

`tx_validity_interval(tx_cbor) -> Json(Int?, Int?)`

### tx_certificates

Returns the list of certificates of the transaction.

`tx_certificates(tx_cbor) -> Json`

### tx_withdrawals

Returns the list of withdrawals of the transaction.

`tx_withdrawals(tx_cbor) -> Json`

### tx_auxiliary_data_hash

Returns the auxiliary data hash of the transaction.

`tx_auxiliary_data_hash(tx_cbor) -> String`

### tx_metadata

Returns the metadata of the transaction.

`tx_metadata(tx_cbor) -> Json`

### tx_script_data_hash

Returns the script data hash of the transaction.

`tx_script_data_hash(tx_cbor) -> String`

### tx_collateral_inputs

Returns the list of collateral inputs of the transaction.

`tx_collateral_inputs(tx_cbor) -> Json`

### tx_required_signers

Returns the list of required signers of the transaction.

`tx_required_signers(tx_cbor) -> Json`

### tx_network_id

Returns the network id of the transaction.

`tx_network_id(tx_cbor) -> Int`

### tx_collateral_return

Returns the list of collateral return of the transaction.

`tx_collateral_return(tx_cbor) -> Json`

### tx_total_collateral

Returns the total collateral of the transaction.

`tx_total_collateral(tx_cbor) -> Int`

### tx_reference_inputs

Returns the list of reference inputs of the transaction.

`tx_reference_inputs(tx_cbor) -> Json`

### tx_witness_set

Returns the witness set of the transaction.

`tx_witness_set(tx_cbor) -> Json`

### tx_redeeemers

Returns the redeemers of the transaction.

`tx_redeeemers(tx_cbor) -> Json`

### tx_plutus_data

Returns the plutus data of the transaction.

`tx_plutus_data(tx_cbor) -> Json`

## Custom Projections

### tx_mint_cip25

Returns the minted tokens of the transaction according to CIP25.

`tx_mint_cip25(tx_cbor) -> Json`

### tx_mint_cip68

Returns the minted tokens of the transaction according to CIP68.

`tx_mint_cip68(tx_cbor) -> Json`

### tx_addresses

Returns the list of addresses of the transaction.

`tx_addresses(tx_cbor) -> Json`

### tx_addresses_bech32

Returns the list of addresses in bech32 of the transaction.

`tx_addresses_bech32(tx_cbor) -> Json`


### tx_output_asset_amount

Returns the amount of an asset in an output.

`tx_output_asset_amount(tx_cbor, subject) -> Int`

### tx_output_to_asset_amount

Returns the amount of an asset in an output to a specific address.

`tx_output_to_asset_amount(tx_cbor, address, subject) -> Int`

### tx_mint_asset_amount

Returns the amount of an asset in a mint.

`tx_mint_asset_amount(tx_cbor, subject) -> Int`

### tx_burn_asset_amount

Returns the amount of an asset in a burn.

`tx_burn_asset_amount(tx_cbor, subject) -> Int`

# FILTERS

### tx_is_valid

Returns true if the transaction is valid.

`tx_is_valid(tx_cbor) -> Bool`

## Has filters

### tx_has_validity_interval

Returns true if the transaction has a validity interval.

`tx_has_validity_interval(tx_cbor) -> Bool`

### tx_has_certificates

Returns true if the transaction has certificates.

`tx_has_certificates(tx_cbor) -> Bool`

### tx_has_withdrawals

Returns true if the transaction has withdrawals.

`tx_has_withdrawals(tx_cbor) -> Bool`

### tx_has_metadata

Returns true if the transaction has metadata.

`tx_has_metadata(tx_cbor) -> Bool`

### tx_has_mint

Returns true if the transaction has minted tokens.

`tx_has_mint(tx_cbor) -> Bool`

### tx_has_burn

Returns true if the transaction has burned tokens.

`tx_has_burn(tx_cbor) -> Bool`

### tx_has_mint_cip25

Returns true if the transaction has minted tokens according to CIP25.

`tx_has_mint_cip25(tx_cbor) -> Bool`

### tx_has_mint_cip68

Returns true if the transaction has minted tokens according to CIP68.

`tx_has_mint_cip68(tx_cbor) -> Bool`

### tx_has_collateral

Returns true if the transaction has collateral.

`tx_has_collateral(tx_cbor) -> Bool`

### tx_has_reference_inputs

Returns true if the transaction has reference inputs.

`tx_has_reference_inputs(tx_cbor) -> Bool`

### tx_has_witness_set

Returns true if the transaction has a witness set.

`tx_has_witness_set(tx_cbor) -> Bool`

### tx_has_required_signers

Returns true if the transaction has required signers.

`tx_has_required_signers(tx_cbor) -> Bool`

### tx_has_reference_inputs

Returns true if the transaction has reference inputs.

`tx_has_reference_inputs(tx_cbor) -> Bool`

## Custom Has filters

### tx_has_policy_id_output

Returns true if the transaction has a policy id.

`tx_has_policy_id_output(tx_cbor, policy_id) -> Bool`

### tx_has_subject_output

Returns true if the transaction has a subject.

`tx_has_subject_output(tx_cbor, subject, amount) -> Bool`

### tx_has_policy_id_mint

Returns true if the transaction has a policy id.

`tx_has_policy_id_mint(tx_cbor, policy_id) -> Bool`

### tx_has_subject_mint

Returns true if the transaction has a subject.

`tx_has_subject_mint(tx_cbor, subject, amount) -> Bool`

### tx_has_policy_id_burn

Returns true if the transaction has a policy id.

`tx_has_policy_id_burn(tx_cbor, policy_id) -> Bool`

### tx_has_plutus_data

Returns true if the transaction has plutus data.

`tx_has_plutus_data(tx_cbor) -> Bool`

### tx_has_redeemers

Returns true if the transaction has redeemers.

`tx_has_redeemers(tx_cbor) -> Bool`

### tx_has_address

Returns true if the transaction has a address.

`tx_has_address(tx_cbor, address) -> Bool`

### tx_has_payment_part

Returns true if the transaction has a payment keyhash.

`tx_has_payment_part(tx_cbor, payment_keyhash) -> Bool`

### tx_has_stake_part

Returns true if the transaction has a stake keyhash.

`tx_has_stake_part(tx_cbor, stake_keyhash) -> Bool`

### tx_has_metadata_label

Returns true if the transaction has a metadata label.

`tx_has_metadata_label(tx_cbor, label) -> Bool`

### tx_has_input

Returns true if the transaction has a input.

`tx_has_input(tx_cbor, tx_hash, output_index) -> Bool`

### tx_has_reference_input

Returns true if the transaction has a reference input.

`tx_has_reference_input(tx_cbor, tx_hash, output_index) -> Bool`

## Is filters


# Utils

### address_payment_part

Returns the payment part of an address.

`address_payment_part(address) -> String`

### address_stake_part

Returns the stake part of an address.

`address_stake_part(address) -> String`

### address_network_id

Returns the network id of an address.

`address_network_id(address) -> Int`

### address_type
Returns the type of an address according to CIP2.

`address_type(address) -> Int`

### pretty_cbor

Returns a pretty printed version of a CBOR.

`pretty_cbor(cbor) -> String`

### slot_to_time

Returns the time of a slot.

`slot_to_time(slot) -> String`

### time_to_slot

Returns the slot of a time.

`time_to_slot(time) -> Int`