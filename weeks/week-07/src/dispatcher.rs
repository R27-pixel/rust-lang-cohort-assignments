use tokio::sync::{mpsc, oneshot};

use crate::{NodeError, NodeRequest, NodeResponse, NodeState};

pub struct NodeCommand {
    pub request: NodeRequest,
    pub response: oneshot::Sender<NodeResponse>,
}

/// Apply one request to node state and return a response.
pub fn handle_request(state: &mut NodeState, request: NodeRequest) -> NodeResponse {
    // Steps:
    // 1. Return `Pong` for `Ping`.
    // 2. Return current height or tip for height/tip requests.
    // 3. Look up blocks for `GetBlock`.
    // 4. Add peers for `AddPeer`.
    // 5. Validate and append blocks for `SubmitBlock`.
    // 6. Never panic for malformed state; return `Rejected` or `Error`.
    match request {
        NodeRequest::Ping => NodeResponse::Pong,
        NodeRequest::Height => NodeResponse::Height(state.height()),
        NodeRequest::GetTip => match state.tip_hash() {
            Some(hash) => NodeResponse::Tip(hash.to_string()),
            None => NodeResponse::Error("missing tip".to_string()),
        },
        NodeRequest::GetBlock(hash) => match state.get_block(&hash) {
            Some(block) => NodeResponse::Block(block.clone()),
            None => NodeResponse::NotFound,
        },
        NodeRequest::AddPeer(address) => NodeResponse::PeerAdded(state.add_peer(&address)),
        NodeRequest::GetPeers => NodeResponse::Peers(state.peer_addresses()),
        NodeRequest::SubmitBlock(block) => {
            let hash = block.hash.clone();
            match state.append_block(block) {
                Ok(()) => NodeResponse::Accepted(hash),
                Err(error) => NodeResponse::Rejected(error.to_string()),
            }
        }
    }
}

/// Run a state manager task that serializes access to `NodeState`.
pub async fn run_state_manager(mut state: NodeState, mut receiver: mpsc::Receiver<NodeCommand>) {
    // Steps:
    // 1. Receive `NodeCommand` values until the channel closes.
    // 2. Handle each request with `handle_request`.
    // 3. Send the response through the command's oneshot sender.
    // 4. Ignore send failures because the caller may have timed out or dropped.
    while let Some(command) = receiver.recv().await {
        let response = handle_request(&mut state, command.request);
        let _ = command.response.send(response);
    }
}

/// Spawn a state manager and return a bounded command sender.
pub fn spawn_state_manager(state: NodeState, capacity: usize) -> mpsc::Sender<NodeCommand> {
    // Steps:
    // 1. Create a bounded channel with the requested capacity.
    // 2. Spawn `run_state_manager(state, receiver)` on Tokio.
    // 3. Return the sender.
    let (sender, receiver) = mpsc::channel(capacity);
    tokio::spawn(run_state_manager(state, receiver));
    sender
}

/// Send one request to the state manager and wait for its response.
pub async fn send_request(
    sender: &mpsc::Sender<NodeCommand>,
    request: NodeRequest,
) -> Result<NodeResponse, NodeError> {
    // Steps:
    // 1. Create a oneshot response channel.
    // 2. Send `NodeCommand { request, response }` through `sender`.
    // 3. Map closed mpsc or oneshot channels to `NodeError::ChannelClosed`.
    // 4. Return the node response.
    let (response_sender, response_receiver) = oneshot::channel();
    sender
        .send(NodeCommand {
            request,
            response: response_sender,
        })
        .await
        .map_err(|_| NodeError::ChannelClosed)?;
    response_receiver
        .await
        .map_err(|_| NodeError::ChannelClosed)
}
