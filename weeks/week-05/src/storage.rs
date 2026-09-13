use std::fs;
use std::path::Path;

use crate::{Block, Blockchain, BtcLibError, Validate};

/// Save a single block as pretty JSON.
pub fn save_block_to_file<P: AsRef<Path>>(block: &Block, path: P) -> Result<(), BtcLibError> {
    // Steps:
    // 1. Validate `block` before saving it.
    block.validate()?;
    // 2. Serialize it with `serde_json::to_string_pretty`.
    let json = serde_json::to_string_pretty(block)?;
    // 3. Write the JSON string to `path` with `std::fs::write`.
    // 4. Convert serde and IO errors through `?`.
    fs::write(path, json)?;

    Ok(())
}

/// Load a single block from JSON and validate it.
pub fn load_block_from_file<P: AsRef<Path>>(path: P) -> Result<Block, BtcLibError> {
    // Steps:
    // 1. Read the file into a string with `std::fs::read_to_string`.
    let content = fs::read_to_string(path)?;
    // 2. Deserialize a `Block` with `serde_json::from_str`.
    let block: Block = serde_json::from_str(&content)?;
    // 3. Validate the loaded block.
    block.validate()?;
    // 4. Return the block only when all steps succeed.
    Ok(block)
}

/// Save a full blockchain snapshot as pretty JSON.
pub fn save_chain_to_file<P: AsRef<Path>>(chain: &Blockchain, path: P) -> Result<(), BtcLibError> {
    // Steps:
    // 1. Validate the chain before saving.
    chain.validate()?;
    // 2. Serialize it with `serde_json::to_string_pretty`.
    let json = serde_json::to_string_pretty(chain)?;
    // 3. Write it to `path`.
    // 4. Propagate errors with `?`.
    fs::write(path, json)?;

    Ok(())
}

/// Load a full blockchain snapshot from JSON and validate it.
pub fn load_chain_from_file<P: AsRef<Path>>(path: P) -> Result<Blockchain, BtcLibError> {
    // Steps:
    // 1. Read the file into a string.
    let content = fs::read_to_string(path)?;
    // 2. Deserialize a `Blockchain`.
    let chain: Blockchain = serde_json::from_str(&content)?;
    // 3. Validate the loaded chain before returning it.
    chain.validate()?;
    // 4. Propagate serde and IO failures through `BtcLibError`.
    Ok(chain)
}
