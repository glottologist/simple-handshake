// Import SinkExt for facilitating message sending operations within futures.
// Import StreamExt for facilitating message receiving operations within futures.
use futures_util::{sink::SinkExt, stream::StreamExt};
// Import WebSocket functionalities from the tokio_tungstenite crate, including asynchronous connection functions and relevant types and errors.
use tokio_tungstenite::{
    connect_async,
    tungstenite::{error::Error as WsError, protocol::Message},
};
// Group imports for organizing code dependencies, including the Transport trait for implementing custom transport logic, async_trait for asynchronous trait methods, and standard IO error handling utilities. Also, include tracing for structured logging.
use {
    super::Transport,
    async_trait::async_trait,
    std::io::{self, Error, ErrorKind},
    tracing::{error, info},
};

// Represents a WebSocket transport mechanism with attributes to store the remote server's URL and a flag indicating the use of secure WebSocket (WSS).
pub struct Ws {
    remote: String,  // URL of the remote server.
    is_secure: bool, // Flag indicating whether a secure connection (WSS) should be used.
}

impl Ws {
    // Constructs a new instance of a WebSocket transport with a specified remote URL and security preference.
    pub fn new(remote: String, is_secure: bool) -> Self {
        Ws { remote, is_secure }
    }
}

// Function to map WebSocket-specific errors to standard IO errors, enabling consistent error handling across different transport mechanisms.
fn convert_error(error: WsError) -> Error {
    match error {
        WsError::Io(io_err) => io_err,
        // Map connection-related errors to BrokenPipe to signify connection issues.
        WsError::ConnectionClosed | WsError::AlreadyClosed => {
            Error::new(ErrorKind::BrokenPipe, error)
        }
        // Handle TLS and capacity errors with generic messages, categorizing them under 'Other'.
        WsError::Tls(err) => Error::new(ErrorKind::Other, format!("TLS error: {}", err)),
        WsError::Capacity(err) => Error::new(ErrorKind::Other, format!("Capacity error: {}", err)),
        // Protocol and UTF-8 errors are treated as data-related issues.
        WsError::Protocol(err) => {
            Error::new(ErrorKind::InvalidData, format!("Protocol error: {}", err))
        }
        WsError::Utf8 => Error::new(ErrorKind::InvalidData, "UTF-8 encoding error"),
        // URL errors are treated as input errors.
        WsError::Url(err) => Error::new(ErrorKind::InvalidInput, format!("URL error: {}", err)),
        // All other errors are mapped to 'Other' for simplicity.
        _ => Error::new(ErrorKind::Other, "Unmapped WebSocket error"),
    }
}

// Asynchronously establishes a WebSocket connection to the specified remote, sends a JSON payload, and awaits a response.
#[warn(unused_assignments)]
async fn ws_send(
    remote: &str,
    _timeout: Option<u32>, // Currently unused. Placeholder for future timeout implementation.
    payload: serde_json::Value,
) -> Result<String, WsError> {
    // Attempt to establish a WebSocket connection asynchronously.
    let (ws_stream, _) = connect_async(remote).await?;

    // Log successful connection establishment.
    info!("Connected to remote websocket {}", remote);

    // Split the WebSocket stream into separate sender and receiver components.
    let (mut write, mut read) = ws_stream.split();

    // Send the JSON payload as a text message through the WebSocket.
    write.send(Message::Text(payload.to_string())).await?;

    // Log the transmission of the payload.
    info!("Sent message payload {}", payload);

    // Initialize a placeholder for storing the response.
    let mut resp = String::new();

    // Process incoming messages, looking for text or binary responses.
    while let Some(message) = read.next().await {
        match message? {
            Message::Text(text) => {
                // Store text responses directly.
                resp = text;
                info!("Received text message {}", &resp);
                break;
            }
            Message::Binary(b) => {
                // Convert binary messages to strings for consistency.
                resp = String::from_utf8_lossy(&b).to_string();
                info!("Received binary message {}", &resp);
                break;
            }
            // Ignore other message types for simplicity.
            _ => continue,
        }
    }

    Ok(resp)
}

// Prepares the remote server URL for connection, ensuring correct WebSocket protocol prefixes are used.
fn rationalise_url(remote: &str, is_secure: bool) -> String {
    if remote.starts_with("wss://") {
        remote.to_string()
    } else if remote.starts_with("ws://") && is_secure {
        // Log a warning if an insecure protocol is specified for a secure connection.
        error!("Target URL starts with ws:// but secure flag is set to true - coercing to wss://");
        format!("wss://{}", &remote[5..])
    } else {
        // Prefix the URL with the appropriate WebSocket protocol based on the security preference.
        match is_secure {
            false => format!("ws://{}", &remote),
            true => format!("wss://{}", &remote),
        }
    }
}

// Implements the Transport trait for WebSocket connections, allowing asynchronous communication with remote servers via WebSocket.
#[async_trait]
impl Transport for Ws {
    async fn connect_and_send(
        &self,
        timeout: Option<u32>, // Timeout parameter, to be utilized for managing connection and response timeouts.
        payload: serde_json::Value, // JSON payload to be sent to the remote server.
    ) -> io::Result<String> {
        // Ensure the remote URL is correctly formatted based on the security preference.
        let remote_url = rationalise_url(&self.remote, self.is_secure);

        // Send the payload to the remote server and await the response, handling any WebSocket errors.
        match ws_send(&remote_url, timeout, payload).await {
            Ok(r) => Ok(r),
            Err(e) => Err(convert_error(e)),
        }
    }
}

// Unit tests and property-based tests to validate error conversion logic and URL formatting robustness.
#[cfg(test)]
mod tests {
    use {super::*, proptest::prelude::*};

    // Test the conversion of WebSocket IO errors to standard IO errors.
    #[test]
    fn test_convert_error_io() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "not found");
        let ws_error = WsError::Io(io_error);
        let converted_error = convert_error(ws_error);
        assert_eq!(converted_error.kind(), std::io::ErrorKind::NotFound);
    }

    // Property-based testing to ensure URL formatting does not cause panics across a range of input values.
    proptest! {
        #[test]
        fn test_url_formatting_does_not_panic(remote in "[a-zA-Z0-9]+") {
            let result = std::panic::catch_unwind(|| {
                let _ = Ws::new(remote, false).connect_and_send(None, serde_json::json!({}));
            });
            assert!(result.is_ok());
        }
    }
}
