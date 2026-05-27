use std::{collections::HashMap, io::Read};

// Name Assignment (variables and constants)
pub const MINING_REWARD: f64 = 0.5;
pub const CURRENT_BLOCK_HEIGHT: u64 = 990000;
pub const BTC_TO_SATS: u64 = 100000000;

#[derive(Debug, Clone, PartialEq)]
pub struct Utxo {
    pub txid: String,
    pub vout: u32,
    pub value: u64,
}

/// Calculate the total Bitcoin reward for a given number of mined blocks.
pub fn calculate_total_reward(blocks_mined: u64) -> f64 {
    blocks_mined as f64 * MINING_REWARD
}

/// Return true if the transaction fee is between 0.00001 and 0.01 BTC.
pub fn is_valid_tx_fee(fee: f64) -> bool {
    if fee >= 0.00001 && fee <= 0.01 {
        true
    } else {
        false
    }
}

/// Return true if the wallet balance is greater than 50.0 BTC.
pub fn is_large_balance(balance: f64) -> bool {
    balance > 50.0
}

/// Return the priority of a transaction ("high", "medium", "low") based on fee rate.
pub fn tx_priority(size_bytes: u64, fee_btc: f64) -> &'static str {
    let calculated_value = fee_btc / size_bytes as f64;

    if calculated_value > 0.00005 {
        "high"
    } else if calculated_value > 0.00001 {
        "medium"
    } else {
        "low"
    }
    // High: > 0.00005, Medium: > 0.00001, otherwise Low
}

/// Return true if the network string equals "mainnet" (case-insensitive).
pub fn is_mainnet(network: &str) -> bool {
    network.to_lowercase() == "mainnet"
}

/// Return true if value is in the inclusive range 100..=200.
pub fn is_in_range(value: i64) -> bool {
    let whatever = 100 <= value && value <= 200;
    whatever
}

/// Return true if both references point to the exact same object in memory.
pub fn is_same_wallet<T>(wallet1: &T, wallet2: &T) -> bool {
    let trans = std::ptr::eq(wallet1, wallet2);
    trans
}

/// Normalize a Bitcoin address by trimming whitespace and lowercasing.
pub fn normalize_address(address: &str) -> String {
    let normalized_val = address.trim().to_lowercase();
    normalized_val
}

/// Append a new UTXO to the list and return the updated list.
pub fn add_utxo(utxos: Vec<Utxo>, new_utxo: Utxo) -> Vec<Utxo> {
    let mut updated_utxos = utxos;
    updated_utxos.push(new_utxo);
    updated_utxos
}

/// Find the first transaction with a fee greater than 0.005 BTC.
pub fn find_high_fee(fee_list: &[f64]) -> Option<(usize, f64)> {
    for (idx, &fee) in fee_list.iter().enumerate() {
        if fee > 0.005 {
            return Some((idx, fee));
        }
    }
    None
}

/// Return basic wallet details as a tuple of (name, balance).
pub fn get_wallet_details() -> (String, f64) {
    let wallet_name = String::from("satoshi_wallet");
    let balance = 50.0;
    (wallet_name, balance)
}

/// Get the status of a transaction from the mempool or "not found".
pub fn get_tx_status(tx_pool: &HashMap<String, String>, txid: &str) -> String {
    if let Some(status) = tx_pool.get(txid) {
        status.clone()
    } else {
        String::from("not found")
    }
}

/// Destructure wallet_info and format a status string.
pub fn unpack_wallet_info(wallet_info: (String, f64)) -> String {
    // the tuple destructuring
    let (name, balance) = wallet_info;
    format!("Wallet {} has balance: {} BTC", name, balance)
}

/// Convert BTC to satoshis (1 BTC = 100,000,000 sats).
pub fn calculate_sats(btc: f64) -> u64 {
    (btc * BTC_TO_SATS as f64) as u64
}

use rand::{thread_rng, Rng};

/// Generate a mock Bitcoin address of length 32 with the given prefix.
pub fn generate_address(prefix: &str) -> String {
    // TODO: Build a random suffix of (32 - prefix.len()) chars from [a-z0-9]
    // TODO: Concatenate prefix + suffix and return

    // initial data for generating random suffix
    let chars: Vec<char> = "asre233443sdsfa34322333".chars().collect();

    // preset to generate random suffix [bond to the random number generator "rng"]
    let mut rng = thread_rng();

    // the random number generator
    let suffix: String = (0..(32 - prefix.len()))
        // to generate random characters from the character set [chars]
        .map(|_| *chars.get(rng.gen_range(0..chars.len())).unwrap())
        .collect();

    // the concatenation
    format!("{}{}", prefix, suffix)
}

/// Validate a Bitcoin block height. Returns (is_valid, message).
pub fn validate_block_height(height: i64) -> (bool, String) {
    // TODO: Check that height is not negative
    // TODO: Check that height is within a realistic range (<= 800_000)
    // TODO: Return (true, "Valid block height") otherwise

    if height < 0 {
        return (false, "negative block height".to_string());
    }
    if height > 800000 {
        return (false, "unrealistic block height -- too high".to_string());
    }
    (true, "Valid block height".to_string())
}

/// Compute the block reward (in sats) for each block height based on the halving schedule.
pub fn halving_schedule(blocks: &[u64]) -> HashMap<u64, u64> {
    // TODO: Base reward is 50 * 100_000_000 sats; halving interval is 210_000 blocks
    // TODO: For each block: halvings = block / 210_000; reward = base >> halvings
    // TODO: Insert (block, reward) into the result HashMap

    //initializations
    let base_reward = 50 * 100_000_000;
    let halving_interval = 210_000;

    let mut result = HashMap::new();

    for block in blocks {
        let split_halving = block / halving_interval;
        let reward = base_reward >> split_halving;

        result.insert(*block, reward);
    }

    result
}

/// Find the UTXO with the smallest value that meets or exceeds target.
pub fn find_utxo_with_min_value(utxos: &[Utxo], target: u64) -> Option<Utxo> {
    // TODO: Filter UTXOs to those with value >= target
    // TODO: Return the one with the smallest value, or None if none qualify

    //filtering out UTXOs with value < target
    let filtered_utxos = utxos.iter().filter(|utxo| utxo.value >= target);

    //finding the UTXO with the smallest value from the filtered list
    if let Some(utxo) = filtered_utxos.min_by_key(|utxo| utxo.value) {
        Some(utxo.clone())
    } else {
        None
    }
}

/// Create a UTXO map from txid, vout, and arbitrary extra string fields.
pub fn create_utxo(
    txid: &str,
    vout: u32,
    extra: HashMap<String, String>,
) -> HashMap<String, String> {
    // TODO: Build a base map with "txid" and "vout" (as string)
    // TODO: Merge extra into the base map and return
    //
    let mut basse_map = HashMap::new();
    basse_map.insert(String::from("txid"), txid.to_string());
    basse_map.insert("vout".to_string(), vout.to_string());

    // let mut merged = basse_map;
    // let mut merged: HashMap<String, String> = basse_map.clone();
    // merged.extend(extra);
    // merged
    basse_map.extend(extra);
    basse_map
}

// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    let transaction_bytes = hex::decode(raw_tx_hex).unwrap();

    let mut byte_slice = transaction_bytes.as_slice();
    let mut transmitter = [0; 4];

    byte_slice.read(&mut transmitter).unwrap();
    let transaction_version = u32::from_le_bytes(transmitter);

    if transaction_bytes.len() < 200 {
        return Err("Transaction data too short".to_string());
    }

    // if {
    //     Err("")
    // }

    Ok(transaction_version)
}
