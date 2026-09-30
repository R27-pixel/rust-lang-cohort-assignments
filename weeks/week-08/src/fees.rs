use crate::{WalletError, WalletUtxo};

/// Derive a deterministic txid for a wallet-created transaction.
///
/// Hash exactly:
/// `wallet-tx:<owner>|to:<recipient>|amount:<amount>|fee:<fee>|inputs:<txid>:<vout>;...`
pub fn derive_wallet_txid(
    owner: &str,
    recipient: &str,
    amount_sats: u64,
    fee_sats: u64,
    inputs: &[WalletUtxo],
) -> String {
    // Steps:
    // 1. Build the exact material documented above.
    // 2. Append input outpoints in the same order as `inputs`.
    // 3. Return `sha256::digest(material)`.
    let mut material =
        format!("wallet-tx:{owner}|to:{recipient}|amount:{amount_sats}|fee:{fee_sats}|inputs:");
    for input in inputs {
        material.push_str(&format!("{}:{};", input.outpoint.txid, input.outpoint.vout));
    }
    sha256::digest(material)
}

/// Calculate fee rate as sats per virtual byte.
pub fn fee_rate_sats_per_vbyte(fee_sats: u64, vbytes: u64) -> Result<f64, WalletError> {
    // Steps:
    // 1. Reject `vbytes == 0` with `InvalidAmount`.
    // 2. Return `fee_sats as f64 / vbytes as f64`.
    if vbytes == 0 {
        return Err(WalletError::InvalidAmount);
    }
    Ok(fee_sats as f64 / vbytes as f64)
}

/// Estimate a simple transaction weight in virtual bytes.
///
/// Formula for this assignment:
/// `10 + inputs * 68 + outputs * 31`
pub fn estimate_transaction_vbytes(input_count: usize, output_count: usize) -> u64 {
    // Steps:
    // 1. Convert counts to `u64`.
    // 2. Return `10 + inputs * 68 + outputs * 31`.
    let input_count = input_count as u64;
    let output_count = output_count as u64;
    10 + input_count * 68 + output_count * 31
}

/// Estimate the fee for a transaction shape.
pub fn estimate_fee_sats(input_count: usize, output_count: usize, sats_per_vbyte: u64) -> u64 {
    // Steps:
    // 1. Calculate vbytes with `estimate_transaction_vbytes`.
    // 2. Multiply by `sats_per_vbyte`.
    // 3. Return the fee.
    estimate_transaction_vbytes(input_count, output_count) * sats_per_vbyte
}
