use crate::{EventLog, NodeClient, NodeStatus, Transaction, Wallet, WalletError};

/// Submit a wallet transaction and update wallet/log state on success.
pub async fn submit_wallet_transaction(
    wallet: &mut Wallet,
    client: &mut NodeClient,
    transaction: Transaction,
    log: &mut EventLog,
) -> Result<String, WalletError> {
    // Steps:
    // 1. Record a `submit` event before contacting the node.
    // 2. Submit the transaction with the client.
    // 3. On success, record the transaction as pending in the wallet.
    // 4. Record an `accepted` event and return the txid.
    // 5. On failure, record a `rejected` event and return the error without mutating wallet UTXOs.
    let txid = transaction.txid.clone();
    log.record("submit", &txid);

    match client.submit_transaction(transaction.clone()).await {
        Ok(submitted_txid) => {
            wallet.record_pending(transaction)?;
            log.record("accepted", &submitted_txid);
            Ok(submitted_txid)
        }
        Err(err) => {
            if let WalletError::NodeRejected(reason) = &err {
                log.record("rejected", reason);
            }
            Err(err)
        }
    }
}

/// Fetch node status and log the sync.
pub async fn sync_wallet_from_node(
    wallet: &mut Wallet,
    client: &NodeClient,
    log: &mut EventLog,
) -> Result<NodeStatus, WalletError> {
    // Steps:
    // 1. Fetch node status.
    // 2. Fetch wallet-related history.
    // 3. Apply each confirmed transaction to the wallet.
    // 4. Record a `sync` event with height and tip hash.
    // 5. Return the node status.
    let status = client.status().await?;
    let history = client.wallet_history(&wallet.owner).await?;

    for transaction in history {
        wallet.apply_confirmed_transaction(transaction);
    }

    log.record("sync", &format!("{}:{}", status.height, status.tip_hash));
    Ok(status)
}

/// Build and submit a transaction in one flow.
pub async fn build_send_and_submit(
    wallet: &mut Wallet,
    client: &mut NodeClient,
    recipient: &str,
    amount_sats: u64,
    fee_sats: u64,
    log: &mut EventLog,
) -> Result<String, WalletError> {
    // Steps:
    // 1. Build the transaction with `wallet.build_transaction`.
    // 2. Submit it with `submit_wallet_transaction`.
    // 3. Return the accepted txid.
    let transaction = wallet.build_transaction(recipient, amount_sats, fee_sats)?;
    submit_wallet_transaction(wallet, client, transaction, log).await
}
