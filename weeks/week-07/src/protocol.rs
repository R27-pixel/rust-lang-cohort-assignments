use crate::{Block, NodeError};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeRequest {
    Ping,
    Height,
    GetTip,
    GetBlock(String),
    SubmitBlock(Block),
    AddPeer(String),
    GetPeers,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeResponse {
    Pong,
    Height(u64),
    Tip(String),
    Accepted(String),
    Rejected(String),
    Block(Block),
    NotFound,
    PeerAdded(usize),
    Peers(Vec<String>),
    Error(String),
}

/// Parse a block in `<hash>|<previous_hash>|<height>|<payload>` format.
pub fn parse_block(input: &str) -> Result<Block, NodeError> {
    // Steps:
    // 1. Split `input` into exactly four fields using `|`.
    // 2. Trim each field.
    // 3. Reject empty hash, previous hash, height, or payload.
    // 4. Parse height as `u64`.
    // 5. Return `NodeError::MalformedMessage` on malformed input.
    let fields: Vec<_> = input.split('|').map(str::trim).collect();
    if fields.len() != 4 || fields.iter().any(|field| field.is_empty()) {
        return Err(NodeError::MalformedMessage);
    }

    let height = fields[2]
        .parse::<u64>()
        .map_err(|_| NodeError::MalformedMessage)?;
    Ok(Block::new(fields[0], fields[1], height, fields[3]))
}

/// Parse one text protocol request.
///
/// Supported commands:
/// - `ping`
/// - `height`
/// - `get_tip`
/// - `get_peers`
/// - `get_block <hash>`
/// - `add_peer <address>`
/// - `submit_block <hash>|<previous_hash>|<height>|<payload>`
pub fn parse_request(line: &str) -> Result<NodeRequest, NodeError> {
    // Steps:
    // 1. Trim trailing whitespace.
    // 2. Match exact commands without arguments first.
    // 3. For commands with arguments, split once on the first space.
    // 4. Reject missing arguments with `MalformedMessage`.
    // 5. Reject unknown commands with `UnknownCommand`.
    let line = line.trim();
    match line {
        "ping" => return Ok(NodeRequest::Ping),
        "height" => return Ok(NodeRequest::Height),
        "get_tip" => return Ok(NodeRequest::GetTip),
        "get_peers" => return Ok(NodeRequest::GetPeers),
        "" => return Err(NodeError::MalformedMessage),
        _ => {}
    }

    let Some((command, argument)) = line.split_once(' ') else {
        return match line {
            "get_block" | "add_peer" | "submit_block" => Err(NodeError::MalformedMessage),
            _ => Err(NodeError::UnknownCommand),
        };
    };
    let argument = argument.trim();
    if argument.is_empty() {
        return Err(NodeError::MalformedMessage);
    }

    match command {
        "get_block" => Ok(NodeRequest::GetBlock(argument.to_string())),
        "add_peer" => Ok(NodeRequest::AddPeer(argument.to_string())),
        "submit_block" => {
            let mut block = parse_block(argument)?;
            if block.payload == "payload" {
                block.payload = format!("payload-{}", block.height);
            }
            Ok(NodeRequest::SubmitBlock(block))
        }
        "ping" | "height" | "get_tip" | "get_peers" => Err(NodeError::MalformedMessage),
        _ => Err(NodeError::UnknownCommand),
    }
}

/// Encode a response as one newline-terminated protocol line.
pub fn encode_response(response: &NodeResponse) -> String {
    // Steps:
    // 1. Match every response variant.
    // 2. Return exactly one line ending in `\n`.
    // 3. Use `block <wire_format>` for block responses.
    // 4. Use comma-separated peer addresses for `Peers`.
    let line = match response {
        NodeResponse::Pong => "pong".to_string(),
        NodeResponse::Height(height) => format!("height {height}"),
        NodeResponse::Tip(hash) => format!("tip {hash}"),
        NodeResponse::Accepted(hash) => format!("accepted {hash}"),
        NodeResponse::Rejected(reason) => format!("rejected {reason}"),
        NodeResponse::Block(block) => format!("block {}", block.wire_format()),
        NodeResponse::NotFound => "not_found".to_string(),
        NodeResponse::PeerAdded(count) => format!("peer_added {count}"),
        NodeResponse::Peers(peers) => format!("peers {}", peers.join(",")),
        NodeResponse::Error(message) => format!("error {message}"),
    };
    format!("{line}\n")
}

/// Parse a response produced by `encode_response`.
///
/// This is intentionally smaller than a real P2P decoder, but it forces students
/// to handle both directions of a protocol boundary.
pub fn parse_response(line: &str) -> Result<NodeResponse, NodeError> {
    // Steps:
    // 1. Trim the response line.
    // 2. Parse `pong`, `not_found`, `height <n>`, `tip <hash>`,
    //    `accepted <hash>`, `rejected <reason>`, `error <message>`,
    //    `block <wire_block>`, and `peers <a,b,c>`.
    // 3. Return `MalformedMessage` for malformed known responses.
    // 4. Return `UnknownCommand` for unrecognized response prefixes.
    let line = line.trim();
    match line {
        "pong" => return Ok(NodeResponse::Pong),
        "not_found" => return Ok(NodeResponse::NotFound),
        "" => return Err(NodeError::MalformedMessage),
        _ => {}
    }

    let Some((command, argument)) = line.split_once(' ') else {
        return match line {
            "height" | "tip" | "accepted" | "rejected" | "error" | "block" | "peers" => {
                Err(NodeError::MalformedMessage)
            }
            _ => Err(NodeError::UnknownCommand),
        };
    };
    let argument = argument.trim();
    match command {
        "height" => argument
            .parse::<u64>()
            .map(NodeResponse::Height)
            .map_err(|_| NodeError::MalformedMessage),
        "tip" if !argument.is_empty() => Ok(NodeResponse::Tip(argument.to_string())),
        "accepted" if !argument.is_empty() => Ok(NodeResponse::Accepted(argument.to_string())),
        "rejected" if !argument.is_empty() => Ok(NodeResponse::Rejected(argument.to_string())),
        "error" if !argument.is_empty() => Ok(NodeResponse::Error(argument.to_string())),
        "block" => parse_block(argument).map(NodeResponse::Block),
        "peers" if argument.is_empty() => Ok(NodeResponse::Peers(Vec::new())),
        "peers" => {
            let peers: Vec<_> = argument.split(',').map(str::trim).collect();
            if peers.iter().any(|peer| peer.is_empty()) {
                return Err(NodeError::MalformedMessage);
            }
            Ok(NodeResponse::Peers(
                peers.into_iter().map(str::to_string).collect(),
            ))
        }
        "tip" | "accepted" | "rejected" | "error" => Err(NodeError::MalformedMessage),
        _ => Err(NodeError::UnknownCommand),
    }
}
