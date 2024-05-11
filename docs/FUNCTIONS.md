# PROJECTIONS

<details>
    <summary>
        <code>tx_hash(tx_cbor: &[u8])</code>
    </summary>

    # Arguments

    * `tx_cbor` - The transaction data in CBOR format.

    # Returns

    The hash of the given transaction data as a string.

    # Example

    select tx_hash(body) from transactions;
</details>

<details>
    <summary>
        <code>tx_inputs(tx_cbor: &[u8])</code>
    </summary>

    # Arguments
    
    * `tx_cbor` - The transaction data in CBOR format.

    # Returns

    An iterator over the inputs of the given transaction data, where each input is represented as a tuple of the input hash and index.

    # Example

    SELECT t.*
    FROM transactions,
    LATERAL tx_inputs(transactions.body) AS t(hash, index)
</details>

<details>
    <summary>
        <code>tx_addresses(tx_cbor: &[u8])</code>
    </summary>

    # Arguments

    * `tx_cbor` - The transaction data in CBOR format.

    # Returns

    A set of addresses involved in the given transaction data.

    # Example

    select tx_addresses("body") from transactions;
</details>

<details>
    <summary>
        <code>tx_plutus_data(tx_cbor: &[u8])</code>
    </summary>

    # Arguments

    * `tx_cbor` - The transaction data in CBOR format.

    # Returns

    The Plutus data of the given transaction data in canonical JSON format.

    # Example

    select tx_plutus_data("body") from transactions;
</details>

<details>
    <summary>
        <code>utxo_address(era: i32, utxo_cbor: &[u8])</code>
    </summary>

    # Arguments

    * `era` - Specifies the era during which the transaction containing this UTXO was executed.

    * `utxo_cbor` - The UTxO data in CBOR format.

    # Returns

    The address of the given UTxO data.

    # Example

    select utxo_address("Era", "Cbor") from utxo;
</details>

<details>
    <summary>
        <code>utxo_lovelace(era: i32, utxo_cbor: &[u8])</code>
    </summary>

    # Arguments

    * `era` - Specifies the era during which the transaction containing this UTXO was executed.

    * `utxo_cbor` - The UTxO data in CBOR format.

    # Returns

    The lovelace amount of the given UTxO data.

    # Example

    select utxo_lovelace("Era", "Cbor") from utxo;
</details>

<details>
    <summary>
        <code>utxo_policy_id_asset_names(era: i32, utxo_cbor: &[u8], policy_id: &[u8])</code>
    </summary>

    # Arguments

    * `era` - Specifies the era during which the transaction containing this UTXO was executed.

    * `utxo_cbor` - The UTxO data in CBOR format.

    * `policy_id` - The policy ID in byte array format.

    # Returns

    A set of asset names associated with the given policy ID in the UTxO data.

    # Example

    select utxo_policy_id_asset_names("Era", "Cbor", encode('policy_hex', 'hex')) from utxo;
</details>

<details>
    <summary>
        <code>utxo_asset_values(era: i32, utxo_cbor: &[u8])</code>
    </summary>

    # Arguments

    * `era` - Specifies the era during which the transaction containing this UTXO was executed.

    * `utxo_cbor` - The UTxO data in CBOR format.

    # Returns

    An iterator over the asset values of the given UTxO data, where each asset value is represented as a tuple of the policy ID, asset name, and amount.

    # Example

    select utxo_asset_values("Era", "Cbor") from utxo;
</details>

<details>
    <summary>
        <code>utxo_policy_id_asset_values(era: i32, utxo_cbor: &[u8], policy_id: &[u8])</code>
    </summary>

    # Arguments

    * `era` - Specifies the era during which the transaction containing this UTXO was executed.

    * `utxo_cbor` - The UTxO data in CBOR format.

    * `policy_id` - The policy ID in byte array format.

    # Returns

    An iterator over the asset values of the given UTxO data associated with the specified policy ID, where each asset value is represented as a tuple of the asset name and amount.

    # Example

    select utxo_policy_id_asset_values("Era", "Cbor", encode('policy_hex', 'hex')) from utxo;
</details>

<details>
    <summary>
        <code>utxo_subject_amount(era: i32, utxo_cbor: &[u8], subject: &[u8])</code>
    </summary>

    # Arguments

    * `era` - Specifies the era during which the transaction containing this UTXO was executed.

    * `utxo_cbor` - The UTxO data in CBOR format.

    * `subject` - The subject in byte array format.

    # Returns

    The amount associated with the given subject in the UTxO data.

    # Example

    select utxo_subject_amount("Era", "Cbor", encode('policy_id_asset_name_hex', 'hex')) from utxo;
</details>

<details>
    <summary>
        <code>utxo_plutus_data(era: i32, utxo_cbor: &[u8])</code>
    </summary>

    # Arguments

    * `era` - Specifies the era during which the transaction containing this UTXO was executed.

    * `utxo_cbor` - The UTxO data in CBOR format.

    # Returns

    The Plutus data of the given UTxO data in canonical JSON format.

    # Example

    select utxo_plutus_data("Era", "Cbor") from utxo;
</details>

# FILTERS

<details>
    <summary>
        <code>utxo_has_policy_id_output(era: i32, utxo_cbor: &[u8], policy_id: &[u8])</code>
    </summary>

    # Arguments

    * `era` - Specifies the era during which the transaction containing this UTXO was executed.

    * `utxo_cbor` - The UTxO data in CBOR format.

    * `address` - The address in byte array format.

    # Returns

    A boolean value indicating whether the given UTxO data has the specified address in its output.

    # Example

    select utxo_has_policy_id_output("Era", "Cbor", encode('policy_hex', 'hex')) from utxo;
</details>

<details>
    <summary>
        <code>utxo_has_address_output(era: i32, utxo_cbor: &[u8], address: &[u8])</code>
    </summary>

    # Arguments

    * `era` - Specifies the era during which the transaction containing this UTXO was executed.

    * `utxo_cbor` - The UTxO data in CBOR format.

    * `address` - The address in byte array format.

    # Returns

    A boolean value indicating whether the given UTxO data has the specified address in its output.

    # Example

    select utxo_has_address_output("Era", "Cbor", address_to_bytes("addr1")) from utxo;
</details>

# UTILITY

<details>
    <summary>
        <code>address_network_id(address: &[u8])</code>
    </summary>

    # Arguments

    * `address` - The address in byte array format.

    # Returns

    The network ID of the given address.

    # Example

    select address_network_id(tx_addresses("body")) from transactions;
</details>

<details>
    <summary>
        <code>address_payment_part(address: &[u8])</code>
    </summary>

    # Arguments

    * `address` - The address in byte array format.

    # Returns

    The payment part of the given address.

    # Example

    select address_payment_part(tx_addresses("body")) from transactions;
</details>

<details>
    <summary>
        <code>address_stake_part(address: &[u8])</code>
    </summary>

    # Arguments

    * `address` - The address in byte array format.

    # Returns

    The stake part of the given address.

    # Example

    select address_stake_part(tx_addresses("body")) from transactions;
</details>

<details>
    <summary>
        <code>address_to_bytes(address: String)</code>
    </summary>

    Returns the byte array representation of the given address string.

    # Arguments

    * `address` - The address in string format.

    # Returns

    The byte array representation of the given address string.

    # Example

    select address_to_bytes(tx_addresses("body")) from transactions;
</details>

<details>
    <summary>
        <code>address_bytes_to_bech32(address: String)</code>
    </summary>

    # Arguments

    * `address` - The address in byte array format.

    # Returns

    The Bech32 representation of the given address.

    # Example

    select address_bytes_to_bech32(tx_addresses("body")) from transactions;
</details>