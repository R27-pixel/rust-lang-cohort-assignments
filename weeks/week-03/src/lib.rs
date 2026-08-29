#![allow(unused_variables)]

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Network {
    Mainnet,
    Testnet,
    Signet,
    Regtest,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TxStatus {
    Spent,
    Unspent,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationError {
    EmptyTxId,
    MissingInputs,
    MissingOutputs,
    ZeroValueOutput,
    EmptyBlock,
    DuplicateTxId,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TxInput {
    pub previous_txid: String,
    pub previous_vout: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TxOutput {
    pub value_sats: u64,
    pub unique_id: Uuid,
    pub recipient: String,
    pub status: TxStatus,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    pub txid: String,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockHeader {
    pub block_hash: String,
    pub previous_block_hash: String,
    pub merkle_root: String,
    pub timestamp: u64,
    pub nonce: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub height: u64,
    pub network: Network,
}

pub trait Identifiable {
    /// Return the stable identifier for this value.
    fn id(&self) -> &str;
}

impl TxInput {
    /// Build a transaction input by copying the previous txid and storing vout.
    pub fn new(previous_txid: &str, previous_vout: u32) -> Self {
        // Steps:
        // 1. Convert `previous_txid` into an owned `String`.
        // 2. Store `previous_vout` unchanged.
        // 3. Return a `TxInput` with both fields filled.
        Self {
            previous_txid: String::from(previous_txid),
            previous_vout,
        }
    }
}

impl TxOutput {
    /// Build a transaction output by copying the recipient and storing value/status.
    pub fn new(value_sats: u64, recipient: &str, status: TxStatus) -> Self {
        Self {
            // Steps:
            // 1. Store `value_sats` unchanged.
            value_sats,
            // 2. Generate a fresh `Uuid` with `Uuid::new_v4()` for `unique_id`.
            unique_id: Uuid::new_v4(),
            // 3. Convert `recipient` into an owned `String`.
            recipient: String::from(recipient),
            // 4. Store `status` unchanged.
            status, // 5. Return a `TxOutput`.
        }
    }

    /// Return true when this output status is `TxStatus::Unspent`.
    pub fn is_unspent(&self) -> bool {
        // Steps:
        // 1. Compare `self.status` with `TxStatus::Unspent`.
        // 2. Return the boolean result.
        self.status == TxStatus::Unspent
    }
}

impl Transaction {
    /// Build a transaction by copying the txid and storing the provided inputs
    /// and outputs.
    pub fn new(txid: &str, inputs: Vec<TxInput>, outputs: Vec<TxOutput>) -> Self {
        Self {
            // Steps:
            // 1. Convert `txid` into an owned `String`.
            txid: txid.to_string(),
            // 2. Move `inputs` and `outputs` into the transaction.
            inputs,
            // 3. Return a `Transaction`.
            outputs,
        }
    }

    /// Return true for the simplified coinbase rule used in this assignment:
    /// txid is `"coinbase"` and there are no inputs.
    pub fn is_coinbase(&self) -> bool {
        // Steps:
        // 1. Check that `self.txid == "coinbase"`.
        // 2. Check that `self.inputs` is empty.
        // 3. Return true only when both checks pass.
        self.inputs.is_empty() && self.txid == "coinbase"
    }

    /// Sum the satoshi value of every output in this transaction.
    pub fn total_output_value(&self) -> u64 {
        // Steps:
        // 1. Start a total at 0.
        // 2. Add each output's `value_sats`.
        // 3. Return the total.
        self.outputs.iter().map(|output| output.value_sats).sum()
    }

    /// Count outputs whose status is `TxStatus::Unspent`.
    pub fn unspent_output_count(&self) -> usize {
        // Steps:
        // 1. Walk through `self.outputs`.
        // 2. Count outputs where `status == TxStatus::Unspent`.
        // 3. Return the count.
        self.outputs
            .iter()
            .filter(|output| output.status == TxStatus::Unspent)
            .count()
    }

    /// Count outputs whose status is `TxStatus::Spent`.
    pub fn spent_output_count(&self) -> usize {
        // Steps:
        // 1. Walk through `self.outputs`.
        // 2. Count outputs where `status == TxStatus::Spent`.
        // 3. Return the count.
        self.outputs
            .iter()
            .filter(|output| output.status == TxStatus::Spent)
            .count()
    }

    /// Validate this transaction using the rules in the README.
    ///
    /// Return the first matching `ValidationError`, otherwise `Ok(())`.
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.txid.is_empty() {
            return Err(ValidationError::EmptyTxId);
        }
        if !self.is_coinbase() && self.inputs.is_empty() {
            return Err(ValidationError::MissingInputs);
        }
        if self.outputs.is_empty() {
            return Err(ValidationError::MissingOutputs);
        }
        if self.outputs.iter().any(|o| o.value_sats == 0) {
            return Err(ValidationError::ZeroValueOutput);
        }
        Ok(())
    }
}

impl Identifiable for Transaction {
    /// Return this transaction's txid.
    fn id(&self) -> &str {
        // Steps:
        // 1. Return `self.txid.as_str()`.
        // 2. Do not allocate a new string.
        self.txid.as_str()
    }
}

impl BlockHeader {
    /// Build a block header by copying the string fields and storing timestamp
    /// and nonce.
    pub fn new(
        block_hash: &str,
        previous_block_hash: &str,
        merkle_root: &str,
        timestamp: u64,
        nonce: u64,
    ) -> Self {
        Self {
            // Steps:
            // 1. Convert `block_hash`, `previous_block_hash`, and `merkle_root`
            //    into owned `String`s.
            block_hash: String::from(block_hash),
            previous_block_hash: String::from(previous_block_hash),
            merkle_root: String::from(merkle_root),
            // 2. Store `timestamp` and `nonce` unchanged.
            timestamp,
            nonce,
            // 3. Return a `BlockHeader`.
        }
    }
}

impl Block {
    /// Build a block from the provided header, transactions, height, and network.
    pub fn new(
        header: BlockHeader,
        transactions: Vec<Transaction>,
        height: u64,
        network: Network,
    ) -> Self {
        // Steps:
        // 1. Move `header` and `transactions` into the block.
        // 2. Store `height` and `network` unchanged.
        // 3. Return a `Block`.
        Self {
            header,
            transactions,
            height,
            network,
        }
    }

    /// Return how many transactions are in this block.
    pub fn transaction_count(&self) -> usize {
        // Steps:
        // 1. Return the length of `self.transactions`.
        self.transactions.len()
    }

    /// Sum the total output value of all transactions in this block.
    pub fn total_output_value(&self) -> u64 {
        // Steps:
        // 1. Start a total at 0.
        // 2. For each transaction, add `transaction.total_output_value()`.
        // 3. Return the total.
        self.transactions
            .iter()
            .map(|tx| tx.total_output_value())
            .sum()
    }

    /// Return the first coinbase transaction in this block, if one exists.
    pub fn coinbase_transaction(&self) -> Option<&Transaction> {
        // Steps:
        // 1. Walk through transactions in order.
        // 2. Return `Some(transaction)` for the first transaction where
        //    `transaction.is_coinbase()` is true.
        // 3. Return `None` if no coinbase transaction exists.
        self.transactions.iter().find(|tx| tx.is_coinbase())
    }

    /// Return a borrowed transaction with the matching txid, if one exists.
    pub fn find_transaction(&self, txid: &str) -> Option<&Transaction> {
        // Steps:
        // 1. Walk through transactions in order.
        // 2. Compare each transaction's `txid` with the requested txid.
        // 3. Return `Some(transaction)` for the first match.
        // 4. Return `None` if no match exists.
        self.transactions.iter().find(|tx| tx.txid == txid)
    }

    /// Validate this block using the rules in the README.
    ///
    /// Return the first matching `ValidationError`, otherwise `Ok(())`.
    pub fn validate(&self) -> Result<(), ValidationError> {
        // Steps:
        // 1. If there are no transactions, return `Err(ValidationError::EmptyBlock)`.
        // 2. Check for duplicate transaction ids and return
        //    `Err(ValidationError::DuplicateTxId)` if any id repeats.
        // 3. Validate each transaction using `transaction.validate()`.
        // 4. Return the first transaction validation error if one occurs.
        // 5. Otherwise return `Ok(())`.
        if self.transactions.is_empty() {
            return Err(ValidationError::EmptyBlock);
        }

        let mut seen = HashSet::new();

        for transaction in &self.transactions {
            if !seen.insert(transaction.id()) {
                return Err(ValidationError::DuplicateTxId);
            }

            transaction.validate()?;
        }

        Ok(())
    }
}

impl Identifiable for Block {
    /// Return this block's block hash.
    fn id(&self) -> &str {
        // Steps:
        // 1. Return `self.header.block_hash.as_str()`.
        // 2. Do not allocate a new string.
        self.header.block_hash.as_str()
    }
}

/// Return the Bitcoin network magic value for a network.
pub fn network_magic(network: Network) -> u32 {
    // Steps:
    // 1. Match on the `Network` enum.
    // 2. Return the exact magic value listed in the README.
    // 3. Keep the values as `u32`.
    match network {
        Network::Mainnet => 0xD9B4BEF9,
        Network::Testnet => 0x0709110B,
        Network::Signet => 0x40CF030A,
        Network::Regtest => 0xDAB5BFFA,
    }
}

/// Convert a known network magic value back to a `Network`.
///
/// Return `None` for unknown magic values.
pub fn network_from_magic(magic: u32) -> Option<Network> {
    // Steps:
    // 1. Compare `magic` against each known magic value.
    // 2. Return `Some(Network::...)` for a match.
    // 3. Return `None` when the value is unknown.
    match magic {
        0xD9B4BEF9 => Some(Network::Mainnet),
        0x0709110B => Some(Network::Testnet),
        0x40CF030A => Some(Network::Signet),
        0xDAB5BFFA => Some(Network::Regtest),
        _ => None,
    }
}

/// Count unspent outputs across all transactions.
pub fn count_unspent_outputs(transactions: &[Transaction]) -> usize {
    // Steps:
    // 1. Walk through every transaction.
    // 2. Add that transaction's unspent output count.
    // 3. Return the combined count.
    transactions
        .iter()
        .map(|tx| tx.unspent_output_count())
        .sum()
}

/// Sum output values whose recipient exactly matches `recipient`.
pub fn total_value_for_recipient(transactions: &[Transaction], recipient: &str) -> u64 {
    // Steps:
    // 1. Walk through every transaction and every output.
    // 2. Add `value_sats` only when `output.recipient == recipient`.
    // 3. Return 0 if no outputs match.
    transactions
        .iter()
        .flat_map(|tx| &tx.outputs)
        .filter(|output| output.recipient == recipient)
        .map(|output| output.value_sats)
        .sum()
}

/// Compare two values through the `Identifiable` trait.
pub fn have_same_id<T: Identifiable, U: Identifiable>(left: &T, right: &U) -> bool {
    // Steps:
    // 1. Call `id()` on both values.
    // 2. Compare the returned string slices.
    // 3. Return true if they are equal.
    left.id() == right.id()
}

/// Collect ids from dynamic trait objects into owned strings.
pub fn collect_ids(items: &[Box<dyn Identifiable>]) -> Vec<String> {
    // Steps:
    // 1. Create a new `Vec<String>`.
    // 2. For each trait object, call `id()`.
    // 3. Convert the borrowed id into an owned `String`.
    // 4. Preserve the input order.
    items.iter().map(|item| item.id().to_string()).collect()
}
