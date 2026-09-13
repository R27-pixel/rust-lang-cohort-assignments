use serde::{Deserialize, Serialize};

use crate::{validate_merkle_root, Block, BtcLibError, Network, Transaction, Validate};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Blockchain {
    pub network: Network,
    pub blocks: Vec<Block>,
}

impl Blockchain {
    /// Create an empty chain for the selected network.
    pub fn new(network: Network) -> Self {
        // Steps:
        // 1. Store `network`.
        // 2. Start with an empty `Vec<Block>`.
        // 3. Return the new `Blockchain`.
        Blockchain {
            network,
            blocks: Vec::new(),
        }
    }

    /// Create a chain that starts with a validated genesis block.
    pub fn from_genesis(genesis: Block) -> Result<Self, BtcLibError> {
        // Steps:
        // 1. Validate the supplied block.
        genesis.validate()?;
        // 2. Validate its merkle root.
        validate_merkle_root(&genesis)?;
        // 3. Create a chain with `genesis.network`.
        let mut chain = Self::new(genesis.network);
        // 4. Push the genesis block into the chain.
        chain.blocks.push(genesis);
        // 5. Return the chain.
        Ok(chain)
    }

    /// Return the current chain height.
    ///
    /// Empty chains return 0. Non-empty chains return the height of the tip block.
    pub fn height(&self) -> u64 {
        // Steps:
        // 1. Look at the tip block with `self.tip()`.
        // 2. Return the tip height when present.
        // 3. Return 0 for an empty chain.
        match self.tip() {
            Some(block) => block.height,
            None => 0,
        }
    }

    /// Return the current tip block.
    pub fn tip(&self) -> Option<&Block> {
        // Steps:
        // 1. Return the last block in `self.blocks`.
        self.blocks.last()
    }

    /// Return the current tip hash, if the chain has a tip.
    pub fn tip_hash(&self) -> Option<&str> {
        // Steps:
        // 1. Get the tip block.
        // 2. Return `Some(tip.header.block_hash.as_str())`.
        // 3. Return `None` for an empty chain.
        match self.tip() {
            Some(tip) => Some(tip.header.block_hash.as_str()),
            None => None,
        }
    }

    /// Append a validated block to the chain.
    pub fn append_block(&mut self, block: Block) -> Result<(), BtcLibError> {
        // Steps:
        // 1. Validate the block and its merkle root.
        block.validate()?;
        validate_merkle_root(&block)?;
        // 2. If the chain is empty, require `block.height == 0`.
        // 3. If the chain is not empty, require:
        //    - `block.header.previous_block_hash` equals the current tip hash.
        //    - `block.height` is exactly current tip height + 1.
        // 4. Push the block and return `Ok(())`.
        // 5. Use `InvalidPreviousHash` for bad linkage or height.
        match self.tip() {
            None => {
                if block.height != 0 {
                    return Err(BtcLibError::InvalidPreviousHash);
                }
            }
            Some(tip) => {
                let prev_hash_matches = block.header.previous_block_hash == tip.header.block_hash;
                let height_matches = block.height == self.height() + 1;

                if !prev_hash_matches || !height_matches {
                    return Err(BtcLibError::InvalidPreviousHash);
                }
            }
        }
        self.blocks.push(block);
        Ok(())
    }

    /// Find a block by its header hash.
    pub fn find_block_by_hash(&self, block_hash: &str) -> Option<&Block> {
        // Steps:
        // 1. Iterate through `self.blocks`.
        // 2. Return the first block whose `header.block_hash` matches.
        // 3. Return `None` when no block matches.
        self.blocks
            .iter()
            .find(|b| b.header.block_hash == block_hash)
    }

    /// Find a transaction anywhere in the chain.
    pub fn find_transaction(&self, txid: &str) -> Option<&Transaction> {
        // Steps:
        // 1. Iterate over blocks in order.
        // 2. Reuse `block.find_transaction(txid)`.
        // 3. Return the first matching transaction.
        // 4. Return `None` if no block contains the transaction.
        self.blocks.iter().find_map(|b| b.find_transaction(txid))
    }

    /// Count all transactions across all blocks.
    pub fn total_transactions(&self) -> usize {
        // Steps:
        // 1. Iterate over every block.
        // 2. Add each block's transaction count.
        // 3. Return the total.
        self.blocks.iter().map(|b| b.transactions.len()).sum()
    }

    /// Validate every block and every link in the chain.
    pub fn validate(&self) -> Result<(), BtcLibError> {
        // Steps:
        // 1. Reject an empty chain with `BtcLibError::EmptyChain`.
        if self.blocks.is_empty() {
            return Err(BtcLibError::EmptyChain);
        }
        // 2. Validate each block and merkle root.
        // 3. For every block after genesis, check previous hash and height.
        // 4. Return the first error.
        for (i, block) in self.blocks.iter().enumerate() {
            block.validate()?;
            validate_merkle_root(block)?;

            if i == 0 {
                if block.height != 0 {
                    return Err(BtcLibError::InvalidPreviousHash);
                }
            } else {
                let prev_block = &self.blocks[i - 1];
                let prev_hash_matches =
                    block.header.previous_block_hash == prev_block.header.block_hash;
                let height_matches = block.height == prev_block.height + 1;

                if !prev_hash_matches || !height_matches {
                    return Err(BtcLibError::InvalidPreviousHash);
                }
            }
        }
        // 5. Return `Ok(())` when the whole chain is valid.
        Ok(())
    }
}

/// Return a lowercase label for the network.
pub fn network_label(network: Network) -> &'static str {
    // Steps:
    // 1. Match every `Network` variant.
    // 2. Return exactly: `mainnet`, `testnet`, `signet`, or `regtest`.
    match network {
        Network::Mainnet => "mainnet",
        Network::Testnet => "testnet",
        Network::Signet => "signet",
        Network::Regtest => "regtest",
    }
}

/// Build a compact chain summary string.
///
/// Use exactly:
/// `network:<network>|height:<height>|blocks:<count>|tip:<tip_hash>|txs:<total_transactions>`
///
/// For an empty chain, use `tip:none`.
pub fn chain_summary(chain: &Blockchain) -> String {
    // Steps:
    // 1. Convert the network to a label with `network_label`.
    // 2. Use `chain.height()` for height.
    // 3. Use `chain.blocks.len()` for block count.
    // 4. Use `chain.tip_hash().unwrap_or("none")` for the tip.
    // 5. Use `chain.total_transactions()` for transaction count.
    // 6. Return the exact format shown above.
    let network = network_label(chain.network);
    let height = chain.height();
    let count = chain.blocks.len();
    let tip = chain.tip_hash().unwrap_or("none");
    let txs = chain.total_transactions();

    format!("network:{network}|height:{height}|blocks:{count}|tip:{tip}|txs:{txs}")
}
