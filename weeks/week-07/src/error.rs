use thiserror::Error;

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum NodeError {
    #[error("malformed message")]
    MalformedMessage,
    #[error("unknown command")]
    UnknownCommand,
    #[error("invalid block")]
    InvalidBlock,
    #[error("block rejected: {0}")]
    BlockRejected(String),
    #[error("missing block")]
    MissingBlock,
    #[error("channel closed")]
    ChannelClosed,
    #[error("io error: {0}")]
    Io(String),
}

impl From<std::io::Error> for NodeError {
    /// Convert socket and file IO errors into a stable node error.
    fn from(error: std::io::Error) -> Self {
        // Steps:
        // 1. Convert `error` into a string.
        // 2. Store it in `NodeError::Io`.
        Self::Io(error.to_string())
    }
}
