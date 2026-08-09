#![allow(unused_variables)]

use std::collections::HashMap;

pub const SATS_PER_BTC: u64 = 100_000_000;
pub const GENESIS_HASH: &str = "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f";
pub const GENESIS_TIMESTAMP: u64 = 1_231_006_505;
pub const GENESIS_REWARD_SATS: u64 = 50 * SATS_PER_BTC;
pub const GENESIS_MESSAGE: &str =
    "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MockTransaction {
    pub txid: String,
    pub sender: String,
    pub recipient: String,
    pub amount_sats: u64,
    pub confirmed: bool,
}

impl MockTransaction {
    /// Build a mock transaction by copying the borrowed string inputs into owned
    /// `String` fields and storing the amount and confirmation flag unchanged.
    pub fn new(
        txid: &str,
        sender: &str,
        recipient: &str,
        amount_sats: u64,
        confirmed: bool,
    ) -> Self {
        // Steps:
        // 1. Convert `txid`, `sender`, and `recipient` into owned `String`s.
        // 2. Store `amount_sats` and `confirmed` exactly as they are passed in.
        // 3. Return a `MockTransaction` with all five fields filled.
        MockTransaction {
            txid:txid.to_string(),
            sender:sender.to_string(),
            recipient:recipient.to_string(),
            amount_sats:amount_sats,
            confirmed:confirmed
        }
    }
}

/// Return the hardcoded Bitcoin genesis block hash.
pub fn genesis_hash() -> &'static str {
    // Steps:
    // 1. Return the `GENESIS_HASH` constant.
    // 2. Do not allocate a new string.
    return GENESIS_HASH
}

/// Return the hardcoded Unix timestamp for the Bitcoin genesis block.
pub fn genesis_timestamp() -> u64 {
    // Steps:
    // 1. Return the `GENESIS_TIMESTAMP` constant.
    return GENESIS_TIMESTAMP
}

/// Return the genesis block reward in satoshis.
pub fn genesis_reward_sats() -> u64 {
    // Steps:
    // 1. Return the `GENESIS_REWARD_SATS` constant.
    // 2. Keep the unit in satoshis, not BTC.
    return GENESIS_REWARD_SATS
}

/// Return the newspaper headline embedded in the genesis block coinbase data.
pub fn genesis_message() -> &'static str {
    // Steps:
    // 1. Return the `GENESIS_MESSAGE` constant.
    // 2. Do not allocate a new string.
    return GENESIS_MESSAGE
}

/// Build a human-readable summary string containing the genesis hash,
/// timestamp, reward, and message.
pub fn genesis_summary() -> String {
    // Steps:
    // 1. Build and return an owned `String`.
    // 2. Use this format:
    //    Genesis block: hash=<hash>, timestamp=<timestamp>, reward_sats=<reward>, message="<message>"
    // 3. Fill each placeholder from the constants above.
    format!(
        "Genesis block: hash={}, timestamp={}, reward_sats={}, message=\"{}\"",
        GENESIS_HASH, GENESIS_TIMESTAMP, GENESIS_REWARD_SATS, GENESIS_MESSAGE
    )
}

/// Calculate the Bitcoin block subsidy for a height.
///
/// Start at 50 BTC, halve every 210,000 blocks, and return zero after
/// 64 or more halvings.

pub fn block_subsidy(height: u64) -> u64 {
    // Steps:
    // 1. Divide `height` by 210_000 to get the halving count.
    // 2. If the halving count is 64 or more, return 0.
    // 3. Otherwise shift or divide the genesis reward by the halving count.
    // 4. Return the result in satoshis.
    let halving_count = height / 210_000;
    if halving_count >= 64 {
        return 0;
    }
    GENESIS_REWARD_SATS >> halving_count
}

/// Format satoshis as a BTC string with exactly eight decimal places.
pub fn format_sats(sats: u64) -> String {
    // Steps:
    // 1. Divide by `SATS_PER_BTC` to get the whole BTC part.
    // 2. Use the remainder for the fractional satoshi part.
    // 3. Return exactly this format: "<whole>.<fraction padded to 8 digits> BTC".
    // 4. Example: 1 sat becomes "0.00000001 BTC".
    let sats_per_btc = sats / 100_000_000;
    let fractional_sats = sats % 100_000_000;
    format!(
        "{:01}.{:08} BTC",
        sats_per_btc,
        fractional_sats
    )
}

/// Count transactions where `confirmed` is true.
pub fn count_confirmed(transactions: &[MockTransaction]) -> usize {
    // Steps:
    // 1. Look through every transaction in the slice.
    // 2. Count only transactions where `confirmed` is true.
    // 3. Return the count.
    return transactions.iter().filter(|tx| tx.confirmed).count();
}

/// Count transactions where `confirmed` is false.
pub fn count_unconfirmed(transactions: &[MockTransaction]) -> usize {
    // Steps:
    // 1. Look through every transaction in the slice.
    // 2. Count only transactions where `confirmed` is false.
    // 3. Return the count.
    return transactions.iter().filter(|tx| !tx.confirmed).count();
}

/// Sum the amount of every transaction, confirmed and unconfirmed.
pub fn total_amount(transactions: &[MockTransaction]) -> u64 {
    // Steps:
    // 1. Start a running total at 0.
    // 2. Add every transaction's `amount_sats`, regardless of confirmation.
    // 3. Return the total.
    return transactions.iter().map(|tx| tx.amount_sats).sum();
}

/// Return the integer average transaction amount.
///
/// Return zero when the input slice is empty.
pub fn average_amount(transactions: &[MockTransaction]) -> u64 {
    // Steps:
    // 1. If the slice is empty, return 0.
    // 2. Otherwise compute `total_amount(transactions) / transactions.len()`.
    // 3. Integer division should round down naturally.
    if transactions.is_empty() {
        return 0;
    }
    let average_amount = total_amount(transactions) / transactions.len() as u64;
    average_amount
}

/// Return cloned transactions whose sender exactly matches `sender`.
///
/// Preserve the original order.
pub fn filter_by_sender(transactions: &[MockTransaction], sender: &str) -> Vec<MockTransaction> {
    // Steps:
    // 1. Create a new `Vec<MockTransaction>`.
    // 2. Walk through the input slice in order.
    // 3. Clone and push transactions whose `sender` equals the requested sender.
    // 4. Return the new vector.
    transactions.iter().filter(|tx| tx.sender == sender).cloned().collect()
}

/// Return cloned transactions whose recipient exactly matches `recipient`.
///
/// Preserve the original order.
pub fn filter_by_recipient(
    transactions: &[MockTransaction],
    recipient: &str,
) -> Vec<MockTransaction> {
    // Steps:
    // 1. Create a new `Vec<MockTransaction>`.
    // 2. Walk through the input slice in order.
    // 3. Clone and push transactions whose `recipient` equals the requested recipient.
    // 4. Return the new vector.
    transactions.iter().filter(|tx| tx.recipient == recipient).cloned().collect()
}

/// Return cloned transactions that are confirmed.
///
/// Preserve the original order.
pub fn filter_confirmed(transactions: &[MockTransaction]) -> Vec<MockTransaction> {
    // Steps:
    // 1. Create a new vector.
    // 2. Add cloned transactions only when `confirmed` is true.
    // 3. Keep the same order as the input slice.
    transactions.iter().filter(|tx| tx.confirmed).cloned().collect()
}

/// Return all transaction ids as owned strings in their original order.
pub fn transaction_ids(transactions: &[MockTransaction]) -> Vec<String> {
    // Steps:
    // 1. Create a new `Vec<String>`.
    // 2. For each transaction, clone its `txid`.
    // 3. Push the cloned txid in the same order as the input.
    transactions.iter().map(|tx| tx.txid.clone()).collect()
}

/// Find the first transaction with a matching txid and return an owned clone.
///
/// Return `None` when no transaction matches.
pub fn find_transaction(transactions: &[MockTransaction], txid: &str) -> Option<MockTransaction> {
    // Steps:
    // 1. Walk through transactions from first to last.
    // 2. When `transaction.txid == txid`, return `Some(transaction.clone())`.
    // 3. If no match is found, return `None`.
    transactions.iter().find(|tx| tx.txid == txid).cloned()
}

/// Return amounts that are strictly greater than `minimum_sats`.
///
/// Preserve the original order.
pub fn amounts_over(transactions: &[MockTransaction], minimum_sats: u64) -> Vec<u64> {
    // Steps:
    // 1. Create a new `Vec<u64>`.
    // 2. For each transaction, compare `amount_sats` with `minimum_sats`.
    // 3. Push only amounts strictly greater than the minimum.
    // 4. Do not include amounts equal to the minimum.
    transactions.iter().filter(|tx| tx.amount_sats > minimum_sats).map(|tx| tx.amount_sats).collect()
}

/// Build a balance map from confirmed transactions only.
///
/// Subtract each confirmed amount from the sender and add it to the recipient.
pub fn build_balances(transactions: &[MockTransaction]) -> HashMap<String, i64> {
    // Steps:
    // 1. Create an empty `HashMap<String, i64>`.
    // 2. Skip every transaction where `confirmed` is false.
    // 3. For confirmed transactions, subtract `amount_sats` from the sender.
    // 4. Add `amount_sats` to the recipient.
    // 5. Convert amounts to `i64` before applying negative changes.
    let mut balances: HashMap<String, i64> = HashMap::new();
    for tx in transactions {
        if tx.confirmed {
            *balances.entry(tx.sender.clone()).or_insert(0) -= tx.amount_sats as i64;
            *balances.entry(tx.recipient.clone()).or_insert(0) += tx.amount_sats as i64;
        }
    }
    balances
}

/// Sum confirmed amounts received by `address`.
pub fn address_received_total(transactions: &[MockTransaction], address: &str) -> u64 {
    // Steps:
    // 1. Look only at confirmed transactions.
    // 2. Add `amount_sats` when `recipient == address`.
    // 3. Return 0 if there are no matches.
    transactions.iter().filter(|tx| tx.confirmed && tx.recipient == address).map(|tx| tx.amount_sats).sum()
}

/// Sum confirmed amounts sent by `address`.
pub fn address_sent_total(transactions: &[MockTransaction], address: &str) -> u64 {
    // Steps:
    // 1. Look only at confirmed transactions.
    // 2. Add `amount_sats` when `sender == address`.
    // 3. Return 0 if there are no matches.
    transactions.iter().filter(|tx| tx.confirmed && tx.sender == address).map(|tx| tx.amount_sats).sum()
}

/// Return confirmed received total minus confirmed sent total for `address`.
pub fn net_balance_change(transactions: &[MockTransaction], address: &str) -> i64 {
    // Steps:
    // 1. Reuse or mirror the received-total and sent-total calculations.
    // 2. Convert both totals to `i64`.
    // 3. Return `received - sent`.
    address_received_total(transactions, address) as i64 - address_sent_total(transactions, address) as i64
}

/// Return true when the transaction sender is exactly `"coinbase"`.
pub fn is_coinbase(transaction: &MockTransaction) -> bool {
    // Steps:
    // 1. Compare `transaction.sender` with the string literal `"coinbase"`.
    // 2. Return the boolean result.
    transaction.sender == "coinbase"
}

/// Classify an amount as `"dust"`, `"micro"`, `"standard"`, or `"large"`.
///
/// Use the thresholds described in this week's README.
pub fn classify_amount(sats: u64) -> &'static str {
    // Steps:
    // 1. Return "dust" for values below 546.
    // 2. Return "micro" for values from 546 up to 99_999.
    // 3. Return "standard" for values from 100_000 up to 99_999_999.
    // 4. Return "large" for values at or above 100_000_000.
    if sats < 546 {
        "dust"
    } else if sats < 100_000 {
        "micro"
    } else if sats < 100_000_000 {
        "standard"
    } else {
        "large"
    }
}
