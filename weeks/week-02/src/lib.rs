#![allow(unused_variables)]

#[derive(Debug, PartialEq, Eq)]
pub struct ParsedOutpoint<'a> {
    pub txid: &'a str,
    pub vout: u32,
}

/// Return true when `input` reads the same forward and backward.
///
/// Ignore ASCII case, whitespace, and punctuation.
pub fn is_palindrome(input: &str) -> bool {
    // Steps:
    // 1. Keep only ASCII alphanumeric characters from `input`.
    // 2. Convert those characters to lowercase.
    // 3. Compare the cleaned sequence with its reverse.
    // 4. Empty cleaned input should count as a palindrome.
    let input_chars = input.chars();
    let mut cleaned = Vec::new();
    for ch in input_chars {
        if ch.is_ascii_alphanumeric() {
            cleaned.push(ch.to_ascii_lowercase());
        }
    }
    return cleaned.iter().eq(cleaned.iter().rev());
}

/// Compute the assignment toy hash.
///
/// Start at zero and, for each byte, update with `hash = hash * 31 + byte`
/// using wrapping arithmetic.
pub fn simple_hash(input: &str) -> u64 {
    // Steps:
    // 1. Start `hash` at 0_u64.
    let mut hash: u64 = 0;
    // 2. Loop over `input.bytes()`.
    for b in input.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(b as u64);
    }
    // 3. For each byte, use `wrapping_mul(31)` and `wrapping_add(byte as u64)`.
    // 4. Return the final hash value.
    hash
}

/// Return `input_sats - output_sats` when inputs cover outputs.
///
/// Return `None` if outputs are larger than inputs.
pub fn calculate_fee(input_sats: u64, output_sats: u64) -> Option<u64> {
    // Steps:
    // 1. Compare `input_sats` and `output_sats`.
    // 2. If outputs are larger, return `None`.
    // 3. Otherwise return `Some(input_sats - output_sats)`.
    if output_sats > input_sats {
        return None;
    }
    return Some(input_sats - output_sats);
}

/// Return the fee rate in sats/vbyte, rounded up.
///
/// Return `None` when `vbytes` is zero.
pub fn fee_rate(fee_sats: u64, vbytes: u64) -> Option<u64> {
    // Steps:
    // 1. If `vbytes` is 0, return `None`.
    if vbytes == 0 {
        return None;
    }
    // 2. Otherwise divide `fee_sats` by `vbytes`, rounding up.
    return Some(fee_sats.div_ceil(vbytes));
    // 3. Return the result in `Some(...)`.
    // 4. Example: 251 sats over 100 vbytes should return 3.
}

/// Return the longer borrowed string slice.
///
/// If both have the same length, return `left`.
pub fn select_longer<'a>(left: &'a str, right: &'a str) -> &'a str {
    // Steps:
    // 1. Compare `left.len()` and `right.len()`.
    // 2. Return `right` only when it is strictly longer.
    // 3. Return `left` when it is longer or equal.
    if right.len() > left.len() {
        return right;
    } else {
        return left;
    }
}

/// Return the first whitespace-separated word from `input`.
///
/// Skip leading whitespace and return an empty slice for empty or all-whitespace
/// input.
pub fn first_word(input: &str) -> &str {
    // Steps:
    // 1. Ignore leading whitespace.
    // 2. Find the first word separated by whitespace.
    // 3. Return a borrowed slice from `input`, not a new `String`.
    // 4. Return "" if no word exists.
    return input.split_whitespace().next().unwrap_or("");
}

/// Return the last whitespace-separated word from `input`.
///
/// Ignore trailing whitespace and return an empty slice for empty or all-whitespace
/// input.
pub fn last_word(input: &str) -> &str {
    // Steps:
    // 1. Ignore trailing whitespace.
    // 2. Find the final word separated by whitespace.
    // 3. Return a borrowed slice from `input`, not a new `String`.
    // 4. Return "" if no word exists.
    return input.split_whitespace().next_back().unwrap_or("");
}

/// Remove `prefix` from the front of `input` when it is present.
///
/// Return the original borrowed `input` slice when the prefix is missing.
pub fn trim_prefix<'a>(input: &'a str, prefix: &str) -> &'a str {
    // Steps:
    // 1. Check whether `input` starts with `prefix`.
    // 2. If it does, return the part of `input` after the prefix.
    // 3. If it does not, return `input` unchanged.
    // 4. An empty prefix should return `input` unchanged.
    input.strip_prefix(prefix).unwrap_or(input)
}

/// Parse a trimmed unsigned satoshi amount.
///
/// Return `None` for empty, negative, or non-numeric input.
pub fn parse_sats(input: &str) -> Option<u64> {
    // Steps:
    // 1. Trim whitespace from `input`.
    // 2. Return `None` if the trimmed string is empty.
    // 3. Try to parse the trimmed string as `u64`.
    // 4. Return `Some(value)` on success, `None` on parse failure.
    input.trim().parse::<u64>().ok()
}

/// Split `input` once on the first colon and trim both sides.
///
/// Return `None` when no colon exists.
pub fn split_once_colon(input: &str) -> Option<(&str, &str)> {
    // Steps:
    // 1. Find the first `:` in `input`.
    // 2. Split into left and right borrowed slices.
    // 3. Trim whitespace from both slices.
    // 4. Return `Some((left, right))`, or `None` if there is no colon.
    let (left, right) = input.split_once(':')?;
    Some((left.trim(), right.trim()))
}

/// Join transaction ids with commas.
///
/// Return an empty string for an empty slice.
pub fn join_txids(txids: &[&str]) -> String {
    // Steps:
    // 1. Keep the txids in their original order.
    // 2. Join them using a comma with no spaces.
    // 3. Example: ["a", "b", "c"] becomes "a,b,c".
    txids.join(",")
}

/// Trim, lowercase, and replace runs of whitespace with single hyphens.
pub fn normalize_label(input: &str) -> String {
    // Steps:
    // 1. Trim leading and trailing whitespace.
    // 2. Split the remaining text on whitespace.
    // 3. Lowercase each word.
    // 4. Join the words with single hyphens.
    // 5. Example: "  Main Wallet  " becomes "main-wallet".
    input
        .split_whitespace()
        .map(|word| word.to_ascii_lowercase())
        .collect::<Vec<_>>()
        .join("-")
}

/// Return true when `needle` exactly matches one of the owned txids.
pub fn contains_txid(txids: &[String], needle: &str) -> bool {
    // Steps:
    // 1. Walk through the borrowed slice of `String`s.
    // 2. Compare each value with `needle`.
    // 3. Return true on the first exact match, otherwise false.
    txids.iter().any(|txid| txid == needle)
}

/// Return a newly allocated string containing `input` followed by `suffix`.
pub fn duplicate_with_suffix(input: &str, suffix: &str) -> String {
    // Steps:
    // 1. Create a new `String`.
    // 2. Put `input` first and `suffix` immediately after it.
    // 3. Do not add spaces or punctuation unless they are part of `suffix`.
    format!("{input}{suffix}")
}

/// Sum the byte lengths of all string slices in `parts`.
pub fn total_byte_len(parts: &[&str]) -> usize {
    // Steps:
    // 1. Start a total at 0.
    // 2. Add `part.len()` for each string slice.
    // 3. Return the total number of bytes.
    parts.iter().map(|part| part.len()).sum()
}

/// Return the borrowed value when present, otherwise return the borrowed default.
pub fn borrowed_or_default<'a>(value: Option<&'a str>, default: &'a str) -> &'a str {
    // Steps:
    // 1. If `value` is `Some(text)`, return `text`.
    // 2. If `value` is `None`, return `default`.
    // 3. Do not allocate a new string.
    value.unwrap_or(default)
}

/// Find `key` in a slice of `(name, amount)` pairs and return the amount.
pub fn lookup_amount(pairs: &[(&str, u64)], key: &str) -> Option<u64> {
    // Steps:
    // 1. Walk through the pairs in order.
    // 2. Compare the name part with `key`.
    // 3. Return `Some(amount)` for the first exact match.
    // 4. Return `None` if the key is missing.
    for &(name, amount) in pairs {
        if name == key {
            return Some(amount);
        }
    }
    None
}

/// Parse an outpoint written as `txid:vout`.
///
/// Trim both fields, borrow the txid from the input, and return `None` for
/// missing separators, empty txids, or non-numeric vouts.
pub fn parse_outpoint(input: &str) -> Option<ParsedOutpoint<'_>> {
    // Steps:
    // 1. Split the input on a single `:` using the helper logic above.
    // 2. Reject the input if the txid side is empty after trimming.
    // 3. Parse the vout side as `u32`.
    // 4. Return `Some(ParsedOutpoint { txid, vout })` on success.
    // 5. Return `None` for any invalid input.
    let (txid, vout_str) = split_once_colon(input)?;
    if txid.is_empty() {
        return None;
    }
    let vout = vout_str.parse::<u32>().ok()?;
    Some(ParsedOutpoint { txid, vout })
}
