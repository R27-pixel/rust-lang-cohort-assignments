use serde::{Deserialize, Serialize};

use crate::MinerError;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TxInput {
    pub previous_txid: String,
    pub previous_vout: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TxOutput {
    pub value_sats: u64,
    pub recipient: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    pub txid: String,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OutPoint {
    pub txid: String,
    pub vout: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Utxo {
    pub outpoint: OutPoint,
    pub value_sats: u64,
    pub recipient: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockHeader {
    pub previous_block_hash: String,
    pub merkle_root: String,
    pub timestamp: u64,
    pub nonce: u64,
    pub difficulty_prefix: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub height: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CandidateBlock {
    pub previous_block_hash: String,
    pub height: u64,
    pub transactions: Vec<Transaction>,
    pub coinbase_recipient: String,
    pub reward_sats: u64,
    pub timestamp: u64,
}

pub trait Hashable {
    /// Return deterministic material that should be hashed.
    fn hash_material(&self) -> String;

    /// Return a SHA-256 hex digest of `hash_material()`.
    fn hash_hex(&self) -> String {
        sha256::digest(self.hash_material())
    }
}

impl TxInput {
    /// Build an input by copying the previous transaction id and output index.
    pub fn new(previous_txid: &str, previous_vout: u32) -> Self {
        Self {
            previous_txid: previous_txid.to_string(),
            previous_vout,
        }
    }

    /// Return this input's previous output as an `OutPoint`.
    pub fn outpoint(&self) -> OutPoint {
        OutPoint {
            txid: self.previous_txid.clone(),
            vout: self.previous_vout,
        }
    }
}

impl TxOutput {
    /// Build an output by copying the recipient and storing the amount.
    pub fn new(value_sats: u64, recipient: &str) -> Self {
        Self {
            value_sats,
            recipient: recipient.to_string(),
        }
    }
}

impl Transaction {
    /// Build a transaction from explicit inputs and outputs.
    pub fn new(txid: &str, inputs: Vec<TxInput>, outputs: Vec<TxOutput>) -> Self {
        Self {
            txid: txid.to_string(),
            inputs,
            outputs,
        }
    }

    /// Build a simplified coinbase transaction.
    ///
    /// Coinbase transactions have no inputs and one output to the miner.
    pub fn coinbase(txid: &str, recipient: &str, reward_sats: u64) -> Self {
        Self::new(txid, vec![], vec![TxOutput::new(reward_sats, recipient)])
    }

    /// Return true when the transaction has no inputs.
    pub fn is_coinbase(&self) -> bool {
        self.inputs.is_empty()
    }

    /// Sum every output amount.
    pub fn total_output_value(&self) -> u64 {
        self.outputs.iter().map(|output| output.value_sats).sum()
    }

    /// Return the fee paid by this transaction using a UTXO lookup.
    pub fn fee_from_utxos<F>(&self, mut lookup: F) -> Result<u64, MinerError>
    where
        F: FnMut(&OutPoint) -> Option<u64>,
    {
        // Steps:
        // 1. Coinbase transactions have fee 0.
        // 2. For regular transactions, look up every input amount.
        // 3. Return `MissingUtxo(label)` if any input is unknown.
        // 4. Sum input values and subtract output value.
        // 5. Return `InvalidSpend(txid)` if outputs exceed inputs.
        if self.is_coinbase() {
            return Ok(0);
        }

        let mut input_total = 0u64;
        for input in &self.inputs {
            let outpoint = input.outpoint();
            let value = lookup(&outpoint).ok_or_else(|| {
                MinerError::MissingUtxo(format!("{}:{}", outpoint.txid, outpoint.vout))
            })?;
            input_total = input_total
                .checked_add(value)
                .ok_or_else(|| MinerError::InvalidSpend(self.txid.clone()))?;
        }

        let output_total = self.total_output_value();
        if output_total > input_total {
            return Err(MinerError::InvalidSpend(self.txid.clone()));
        }

        Ok(input_total - output_total)
    }
}

impl Hashable for Transaction {
    /// Return deterministic transaction material.
    ///
    /// Use exactly:
    /// `tx:<txid>|inputs:<previous_txid>:<previous_vout>;...|outputs:<value>:<recipient>;...`
    fn hash_material(&self) -> String {
        // Steps:
        // 1. Start with `tx:<txid>|inputs:`.
        // 2. Append each input as `<previous_txid>:<previous_vout>;`.
        // 3. Append `|outputs:`.
        // 4. Append each output as `<value_sats>:<recipient>;`.
        // 5. Return the final string.
        let mut material = format!("tx:{}|inputs:", self.txid);
        for input in &self.inputs {
            material.push_str(&format!("{}:{};", input.previous_txid, input.previous_vout));
        }
        material.push_str("|outputs:");
        for output in &self.outputs {
            material.push_str(&format!("{}:{};", output.value_sats, output.recipient));
        }
        material
    }
}

impl Hashable for Block {
    /// Return deterministic block material.
    ///
    /// Use exactly:
    /// `block:<previous_hash>|height:<height>|merkle:<merkle>|time:<timestamp>|nonce:<nonce>|txs:<txid>;...`
    fn hash_material(&self) -> String {
        // Steps:
        // 1. Start with previous hash, height, merkle root, timestamp, and nonce.
        // 2. Append every transaction id followed by `;`.
        // 3. Return the final string.
        let mut material = format!(
            "block:{}|height:{}|merkle:{}|time:{}|nonce:{}|txs:",
            self.header.previous_block_hash,
            self.height,
            self.header.merkle_root,
            self.header.timestamp,
            self.header.nonce
        );
        for transaction in &self.transactions {
            material.push_str(&format!("{};", transaction.txid));
        }
        material
    }
}
