use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;

use crate::{
    calculate_merkle_root, hash_candidate, hash_meets_difficulty, Block, BlockHeader,
    CandidateBlock, Mempool, MinerError, Transaction,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MiningConfig {
    pub difficulty_prefix: String,
    pub start_nonce: u64,
    pub max_nonce: u64,
    pub worker_count: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MinedNonce {
    pub nonce: u64,
    pub hash: String,
    pub attempts: u64,
    pub worker_id: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MiningReport {
    pub block: Block,
    pub nonce: u64,
    pub hash: String,
    pub attempts: u64,
    pub worker_count: usize,
}

/// Build a candidate from a mempool and remove selected transactions.
pub fn build_candidate_from_mempool(
    mempool: &mut Mempool,
    previous_block_hash: &str,
    height: u64,
    coinbase_recipient: &str,
    reward_sats: u64,
    timestamp: u64,
    max_mempool_txs: usize,
) -> CandidateBlock {
    // Steps:
    // 1. Create a coinbase transaction with txid `coinbase-<height>`.
    // 2. Drain up to `max_mempool_txs` transactions from the mempool.
    // 3. Put coinbase first, then drained transactions.
    // 4. Copy the previous hash and coinbase recipient into the candidate.
    // 5. Store reward, timestamp, and height unchanged.
    let coinbase_transaction = Transaction::coinbase(
        &format!("coinbase-{height}"),
        coinbase_recipient,
        reward_sats,
    );
    let mut transactions = Vec::new();
    transactions.push(coinbase_transaction);
    transactions.extend(mempool.drain_for_candidate(max_mempool_txs));

    CandidateBlock {
        previous_block_hash: previous_block_hash.to_string(),
        height,
        transactions,
        coinbase_recipient: coinbase_recipient.to_string(),
        reward_sats,
        timestamp,
    }
}

/// Build a concrete block from a candidate and nonce.
pub fn build_candidate_block(
    candidate: &CandidateBlock,
    nonce: u64,
    difficulty_prefix: &str,
) -> Result<Block, MinerError> {
    // Steps:
    // 1. Reject an empty transaction list with `EmptyCandidate`.
    // 2. Calculate the merkle root.
    // 3. Build a `BlockHeader` with candidate fields, nonce, and difficulty prefix.
    // 4. Move or clone the candidate transactions into a `Block`.
    // 5. Return the block.
    if candidate.transactions.is_empty() {
        return Err(MinerError::EmptyCandidate);
    }

    let merkle_root = calculate_merkle_root(&candidate.transactions)?;
    let header = BlockHeader {
        previous_block_hash: candidate.previous_block_hash.clone(),
        merkle_root,
        timestamp: candidate.timestamp,
        nonce,
        difficulty_prefix: difficulty_prefix.to_string(),
    };

    Ok(Block {
        header,
        height: candidate.height,
        transactions: candidate.transactions.clone(),
    })
}

/// Split an inclusive nonce range across workers.
pub fn split_nonce_ranges(
    start_nonce: u64,
    max_nonce: u64,
    worker_count: usize,
) -> Result<Vec<(u64, u64)>, MinerError> {
    // Steps:
    // 1. Reject `worker_count == 0` and `start_nonce > max_nonce` with `InvalidDifficulty`.
    // 2. Treat the range as inclusive.
    // 3. Split the range as evenly as possible.
    // 4. Give one extra nonce to earlier workers when the range does not divide evenly.
    // 5. Do not return empty ranges.
    if worker_count == 0 || start_nonce > max_nonce {
        return Err(MinerError::InvalidDifficulty);
    }

    let total = (max_nonce - start_nonce + 1) as usize;
    let worker_count = worker_count.min(total);
    let base = total / worker_count;
    let remainder = total % worker_count;

    let mut ranges = Vec::with_capacity(worker_count);
    let mut current = start_nonce;
    for worker_index in 0..worker_count {
        let size = base + usize::from(worker_index < remainder);
        let end = current + size as u64 - 1;
        ranges.push((current, end));
        current = end + 1;
    }

    Ok(ranges)
}

/// Search one inclusive nonce range.
pub fn mine_range(
    candidate: &CandidateBlock,
    difficulty_prefix: &str,
    start_nonce: u64,
    end_nonce: u64,
    worker_id: usize,
) -> Result<Option<MinedNonce>, MinerError> {
    // Steps:
    // 1. Reject invalid ranges with `InvalidDifficulty`.
    // 2. For every nonce from start to end, hash the candidate.
    // 3. Count every attempted nonce.
    // 4. Return `Ok(Some(MinedNonce))` for the first hash that meets difficulty.
    // 5. Return `Ok(None)` if the range has no solution.
    if start_nonce > end_nonce {
        return Err(MinerError::InvalidDifficulty);
    }

    let mut attempts = 0u64;
    for nonce in start_nonce..=end_nonce {
        attempts += 1;
        let hash = hash_candidate(candidate, nonce)?;
        if hash_meets_difficulty(&hash, difficulty_prefix)? {
            return Ok(Some(MinedNonce {
                nonce,
                hash,
                attempts,
                worker_id,
            }));
        }
    }

    Ok(None)
}

/// Mine using a single worker over the configured nonce range.
pub fn mine_single_threaded(
    candidate: &CandidateBlock,
    config: &MiningConfig,
) -> Result<MiningReport, MinerError> {
    // Steps:
    // 1. Search from `config.start_nonce` through `config.max_nonce`.
    // 2. If a nonce is found, build the block with that nonce.
    // 3. Return a `MiningReport` with `worker_count` set to 1.
    // 4. Return `NoSolution` when the range has no valid nonce.
    match mine_range(
        candidate,
        &config.difficulty_prefix,
        config.start_nonce,
        config.max_nonce,
        0,
    )? {
        Some(mined) => {
            let block = build_candidate_block(candidate, mined.nonce, &config.difficulty_prefix)?;
            Ok(MiningReport {
                block,
                nonce: mined.nonce,
                hash: mined.hash,
                attempts: mined.attempts,
                worker_count: 1,
            })
        }
        None => Err(MinerError::NoSolution),
    }
}

/// Mine using several workers and return the first solution reported.
pub fn mine_multi_threaded(
    candidate: CandidateBlock,
    config: MiningConfig,
) -> Result<MiningReport, MinerError> {
    // Steps:
    // 1. Split the nonce range with `split_nonce_ranges`.
    // 2. Spawn one thread per range.
    // 3. Use a channel to report the first found nonce.
    // 4. Use shared cancellation so workers can stop after a solution is found.
    // 5. Join worker threads before returning.
    // 6. Return `NoSolution` if no worker finds a nonce.
    let ranges = split_nonce_ranges(config.start_nonce, config.max_nonce, config.worker_count)?;
    let (tx, rx) = mpsc::channel::<Result<MinedNonce, MinerError>>();
    let cancelled = Arc::new(AtomicBool::new(false));
    let mut handles = Vec::with_capacity(ranges.len());

    for (worker_id, (start_nonce, end_nonce)) in ranges.into_iter().enumerate() {
        let candidate_for_worker = candidate.clone();
        let difficulty_prefix = config.difficulty_prefix.clone();
        let tx = tx.clone();
        let cancelled = Arc::clone(&cancelled);

        handles.push(thread::spawn(move || {
            let mut attempts = 0u64;
            for nonce in start_nonce..=end_nonce {
                if cancelled.load(Ordering::SeqCst) {
                    break;
                }
                attempts += 1;
                let hash = match hash_candidate(&candidate_for_worker, nonce) {
                    Ok(hash) => hash,
                    Err(_) => continue,
                };
                match hash_meets_difficulty(&hash, &difficulty_prefix) {
                    Ok(true) => {
                        cancelled.store(true, Ordering::SeqCst);
                        let _ = tx.send(Ok(MinedNonce {
                            nonce,
                            hash,
                            attempts,
                            worker_id,
                        }));
                        break;
                    }
                    Ok(false) => {}
                    Err(_) => {}
                }
            }
        }));
    }
    drop(tx);

    let mined = rx.recv().map_err(|_| MinerError::NoSolution)??;
    for handle in handles {
        let _ = handle.join();
    }

    let block = build_candidate_block(&candidate, mined.nonce, &config.difficulty_prefix)?;
    Ok(MiningReport {
        block,
        nonce: mined.nonce,
        hash: mined.hash,
        attempts: mined.attempts,
        worker_count: config.worker_count,
    })
}

/// Build a compact mining progress line.
///
/// Use exactly:
/// `workers:<worker_count>|nonce:<nonce>|attempts:<attempts>|hash:<hash>`
pub fn progress_line(report: &MiningReport) -> String {
    // Steps:
    // 1. Read fields from `report`.
    // 2. Return the exact format documented above.
    format!(
        "workers:{}|nonce:{}|attempts:{}|hash:{}",
        report.worker_count, report.nonce, report.attempts, report.hash
    )
}
