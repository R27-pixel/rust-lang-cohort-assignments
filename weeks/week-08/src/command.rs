use crate::WalletError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WalletCommand {
    Balance,
    History,
    Sync,
    Send {
        recipient: String,
        amount_sats: u64,
        fee_sats: u64,
    },
}

/// Parse a small wallet command language.
///
/// Supported commands:
/// - `balance`
/// - `history`
/// - `sync`
/// - `send <recipient> <amount_sats> <fee_sats>`
pub fn parse_wallet_command(input: &str) -> Result<WalletCommand, WalletError> {
    // Steps:
    // 1. Trim the input and split on whitespace.
    // 2. Parse the three single-word commands exactly.
    // 3. Parse `send` with exactly three arguments.
    // 4. Reject empty recipient, zero amount, invalid amount, invalid fee,
    //    or extra/missing arguments with `MalformedData` or `InvalidAmount`.
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    match parts.as_slice() {
        [] => Err(WalletError::MalformedData),
        ["balance"] => Ok(WalletCommand::Balance),
        ["history"] => Ok(WalletCommand::History),
        ["sync"] => Ok(WalletCommand::Sync),
        ["send", recipient, amount, fee] => {
            if recipient.is_empty() {
                return Err(WalletError::InvalidAmount);
            }

            let amount_sats = amount
                .parse::<u64>()
                .map_err(|_| WalletError::MalformedData)?;
            if amount_sats == 0 {
                return Err(WalletError::InvalidAmount);
            }

            let fee_sats = fee.parse::<u64>().map_err(|_| WalletError::MalformedData)?;
            Ok(WalletCommand::Send {
                recipient: (*recipient).to_string(),
                amount_sats,
                fee_sats,
            })
        }
        _ => Err(WalletError::MalformedData),
    }
}

/// Render a command into a compact log-friendly label.
pub fn wallet_command_label(command: &WalletCommand) -> String {
    // Steps:
    // 1. Return `balance`, `history`, or `sync` for simple commands.
    // 2. For send, return exactly `send:<recipient>:<amount_sats>:<fee_sats>`.
    match command {
        WalletCommand::Balance => "balance".to_string(),
        WalletCommand::History => "history".to_string(),
        WalletCommand::Sync => "sync".to_string(),
        WalletCommand::Send {
            recipient,
            amount_sats,
            fee_sats,
        } => format!("send:{recipient}:{amount_sats}:{fee_sats}"),
    }
}
