use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, oneshot};

use crate::{encode_response, parse_request, send_request, NodeCommand, NodeError, NodeResponse};

/// Parse a line, send it to the state manager, and encode the response.
pub async fn handle_line(
    sender: &mpsc::Sender<NodeCommand>,
    line: &str,
) -> Result<String, NodeError> {
    // Steps:
    // 1. Parse the line with `parse_request`.
    // 2. Send the request with `send_request`.
    // 3. Encode the response with `encode_response`.
    let request = parse_request(line)?;
    let response = send_request(sender, request).await?;
    Ok(encode_response(&response))
}

/// Handle one TCP connection line-by-line.
pub async fn handle_connection(
    stream: TcpStream,
    sender: mpsc::Sender<NodeCommand>,
) -> Result<(), NodeError> {
    // Steps:
    // 1. Split the stream into a reader and writer.
    // 2. Read newline-delimited requests.
    // 3. For each request, call `handle_line`.
    // 4. Write the encoded response back to the socket.
    // 5. For parse errors, write an encoded `NodeResponse::Error`.
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        line.clear();
        if reader.read_line(&mut line).await? == 0 {
            break;
        }

        let response = match handle_line(&sender, &line).await {
            Ok(response) => response,
            Err(error) => encode_response(&NodeResponse::Error(error.to_string())),
        };
        writer.write_all(response.as_bytes()).await?;
    }
    Ok(())
}

/// Run a TCP server until the shutdown signal is received.
pub async fn run_tcp_server(
    listener: TcpListener,
    sender: mpsc::Sender<NodeCommand>,
    mut shutdown: oneshot::Receiver<()>,
) -> Result<(), NodeError> {
    // Steps:
    // 1. Loop with `tokio::select!`.
    // 2. Accept inbound connections from `listener`.
    // 3. Spawn `handle_connection` for each connection.
    // 4. Break the loop when `shutdown` resolves.
    // 5. Return `Ok(())` after graceful shutdown.
    loop {
        tokio::select! {
            _ = &mut shutdown => break,
            accepted = listener.accept() => {
                let (stream, _) = accepted?;
                let sender = sender.clone();
                tokio::spawn(async move {
                    let _ = handle_connection(stream, sender).await;
                });
            }
        }
    }
    Ok(())
}
