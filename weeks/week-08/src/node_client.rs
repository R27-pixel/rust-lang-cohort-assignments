use crate::{NodeStatus, Transaction, WalletError};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeClient {
    pub accepted_transactions: Vec<Transaction>,
    pub height: u64,
    pub tip_hash: String,
    pub history: Vec<Transaction>,
    pub reject_next: Option<String>,
}

impl NodeClient {
    /// Create a mock node client for assignment tests.
    pub fn new(height: u64, tip_hash: &str) -> Self {
        // Steps:
        // 1. Store height and tip hash.
        // 2. Start with empty accepted transaction and history lists.
        // 3. Set `reject_next` to `None`.
        Self {
            accepted_transactions: Vec::new(),
            height,
            tip_hash: tip_hash.to_string(),
            history: Vec::new(),
            reject_next: None,
        }
    }

    /// Submit a transaction to the node.
    pub async fn submit_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<String, WalletError> {
        // Steps:
        // 1. If `reject_next` contains a reason, take it and return `NodeRejected(reason)`.
        // 2. Otherwise clone or move the transaction into `accepted_transactions`.
        // 3. Also add it to node history.
        // 4. Return the accepted txid.
        if let Some(reason) = self.reject_next.take() {
            return Err(WalletError::NodeRejected(reason));
        }

        let txid = transaction.txid.clone();
        self.accepted_transactions.push(transaction.clone());
        self.history.push(transaction);
        Ok(txid)
    }

    /// Fetch current node status.
    pub async fn status(&self) -> Result<NodeStatus, WalletError> {
        // Steps:
        // 1. Return `NodeStatus { height, tip_hash }`.
        Ok(NodeStatus {
            height: self.height,
            tip_hash: self.tip_hash.clone(),
        })
    }

    /// Return transactions from node history that involve `owner`.
    pub async fn wallet_history(&self, owner: &str) -> Result<Vec<Transaction>, WalletError> {
        // Steps:
        // 1. Iterate over node history.
        // 2. Keep transactions where any output pays `owner`.
        // 3. Return the matching transactions.
        Ok(self
            .history
            .iter()
            .filter(|transaction| transaction.pays_owner(owner))
            .cloned()
            .collect())
    }
}
