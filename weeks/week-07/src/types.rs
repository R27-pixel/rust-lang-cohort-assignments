use serde::{Deserialize, Serialize};

use crate::NodeError;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Block {
    pub hash: String,
    pub previous_hash: String,
    pub height: u64,
    pub payload: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PeerInfo {
    pub address: String,
    pub last_seen_height: u64,
}

impl Block {
    /// Build a block from borrowed string fields.
    pub fn new(hash: &str, previous_hash: &str, height: u64, payload: &str) -> Self {
        Self {
            hash: hash.to_string(),
            previous_hash: previous_hash.to_string(),
            height,
            payload: payload.to_string(),
        }
    }

    /// Build a deterministic genesis block for tests and local runs.
    pub fn genesis() -> Self {
        Self::new("genesis", "0", 0, "genesis")
    }

    /// Return the wire representation used by this assignment.
    ///
    /// Use exactly: `<hash>|<previous_hash>|<height>|<payload>`.
    pub fn wire_format(&self) -> String {
        format!(
            "{}|{}|{}|{}",
            self.hash, self.previous_hash, self.height, self.payload
        )
    }

    /// Validate this block against an optional current tip.
    pub fn validate_against_tip(&self, tip: Option<&Block>) -> Result<(), NodeError> {
        // Steps:
        // 1. Reject empty hash or payload with `InvalidBlock`.
        // 2. For genesis (`tip == None`), require height 0 and previous hash `0`.
        // 3. For non-genesis, require previous hash equal to tip hash.
        // 4. Require height equal to tip height + 1.
        // 5. Return `Ok(())` if all checks pass.
        if self.hash.is_empty() || self.payload.is_empty() {
            return Err(NodeError::InvalidBlock);
        }

        match tip {
            None if self.height == 0 && self.previous_hash == "0" => Ok(()),
            None => Err(NodeError::InvalidBlock),
            Some(tip)
                if self.previous_hash == tip.hash
                    && tip.height.checked_add(1) == Some(self.height) =>
            {
                Ok(())
            }
            Some(_) => Err(NodeError::InvalidBlock),
        }
    }
}
