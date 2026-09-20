use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{Block, MinerError, OutPoint, Transaction, TxInput, TxOutput, Utxo};

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct UtxoSet {
    pub entries: BTreeMap<OutPoint, Utxo>,
}

impl UtxoSet {
    /// Create an empty UTXO set.
    pub fn new() -> Self {
        // Steps:
        // 1. Create an empty `BTreeMap`.
        // 2. Store it in `UtxoSet`.
        // 3. Return the set.
        Self {
            entries: BTreeMap::new(),
        }
    }

    /// Return how many unspent outputs are tracked.
    pub fn len(&self) -> usize {
        // Steps:
        // 1. Return `self.entries.len()`.
        self.entries.len()
    }

    /// Return true when the set has no entries.
    pub fn is_empty(&self) -> bool {
        // Steps:
        // 1. Return whether `self.entries` is empty.
        self.entries.is_empty()
    }

    /// Insert one transaction output into the UTXO set.
    pub fn insert_output(
        &mut self,
        txid: &str,
        vout: u32,
        output: &TxOutput,
    ) -> Result<(), MinerError> {
        // Steps:
        // 1. Build an `OutPoint` from `txid` and `vout`.
        // 2. If the outpoint already exists, return `DuplicateUtxo(outpoint_label)`.
        // 3. Build a `Utxo` from the outpoint and output.
        // 4. Insert it into the map and return `Ok(())`.
        let outpoint = OutPoint {
            txid: txid.to_string(),
            vout,
        };
        if self.entries.contains_key(&outpoint) {
            return Err(MinerError::DuplicateUtxo(outpoint_label(&outpoint)));
        }

        let utxo = Utxo {
            outpoint: outpoint.clone(),
            value_sats: output.value_sats,
            recipient: output.recipient.clone(),
        };
        self.entries.insert(outpoint, utxo);
        Ok(())
    }

    /// Return a borrowed UTXO by outpoint.
    pub fn get(&self, outpoint: &OutPoint) -> Option<&Utxo> {
        // Steps:
        // 1. Return `self.entries.get(outpoint)`.
        self.entries.get(outpoint)
    }

    /// Spend one input by removing its referenced UTXO.
    pub fn spend_input(&mut self, input: &TxInput) -> Result<Utxo, MinerError> {
        // Steps:
        // 1. Convert the input into an outpoint.
        // 2. Remove the matching UTXO from the map.
        // 3. Return `Ok(utxo)` if present.
        // 4. Return `MissingUtxo(outpoint_label)` if absent.
        let outpoint = input.outpoint();
        self.entries
            .remove(&outpoint)
            .ok_or_else(|| MinerError::MissingUtxo(outpoint_label(&outpoint)))
    }

    /// Apply a transaction to the set.
    ///
    /// Regular transactions spend all inputs and then insert all outputs.
    /// Coinbase transactions only insert outputs.
    pub fn apply_transaction(&mut self, transaction: &Transaction) -> Result<(), MinerError> {
        // Steps:
        // 1. Before mutating, check that every non-coinbase input exists.
        // 2. Spend every input for regular transactions.
        // 3. Insert every output using its vector index as `vout`.
        // 4. Return the first error without partially applying an invalid transaction.
        if !transaction.is_coinbase() {
            for input in &transaction.inputs {
                let outpoint = input.outpoint();
                if !self.entries.contains_key(&outpoint) {
                    return Err(MinerError::MissingUtxo(outpoint_label(&outpoint)));
                }
            }
        }

        if transaction.is_coinbase() {
            for (index, output) in transaction.outputs.iter().enumerate() {
                self.insert_output(&transaction.txid, index as u32, output)?;
            }
            return Ok(());
        }

        for input in &transaction.inputs {
            let outpoint = input.outpoint();
            let _ = self
                .entries
                .remove(&outpoint)
                .ok_or_else(|| MinerError::MissingUtxo(outpoint_label(&outpoint)))?;
        }

        for (index, output) in transaction.outputs.iter().enumerate() {
            self.insert_output(&transaction.txid, index as u32, output)?;
        }

        Ok(())
    }

    /// Apply every transaction in a block-like slice in order.
    pub fn apply_transactions(&mut self, transactions: &[Transaction]) -> Result<(), MinerError> {
        // Steps:
        // 1. Iterate through `transactions`.
        // 2. Apply each transaction to the UTXO set.
        // 3. Stop and return the first error.
        // 4. Return `Ok(())` if every transaction applies.
        for transaction in transactions {
            self.apply_transaction(transaction)?;
        }
        Ok(())
    }

    /// Apply every transaction in a block in order.
    pub fn apply_block(&mut self, block: &Block) -> Result<(), MinerError> {
        // Steps:
        // 1. Reuse `apply_transactions`.
        // 2. Pass `block.transactions` as the transaction slice.
        self.apply_transactions(&block.transactions)
    }

    /// Sum all UTXOs for one recipient.
    pub fn total_for_recipient(&self, recipient: &str) -> u64 {
        // Steps:
        // 1. Iterate through all UTXOs.
        // 2. Add amounts only when `utxo.recipient == recipient`.
        // 3. Return the total.
        self.entries
            .values()
            .filter(|utxo| utxo.recipient == recipient)
            .map(|utxo| utxo.value_sats)
            .sum()
    }
}

/// Convert an outpoint into `txid:vout`.
pub fn outpoint_label(outpoint: &OutPoint) -> String {
    format!("{}:{}", outpoint.txid, outpoint.vout)
}
