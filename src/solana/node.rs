// Use the `async_trait` crate to enable asynchronous methods in traits,
// which are not supported natively by Rust.
use async_trait::async_trait;
// Import the necessary types from the Tokio crate for asynchronous I/O operations.
use tokio::io;

/// The `Handshake` trait defines an asynchronous protocol handshake operation.
///
/// Implementors of this trait must provide an asynchronous `shake` method,
/// which may optionally accept a timeout specified as an `Option<u32>`.
/// The method returns a `Result<String, io::Error>`, indicating the outcome
/// of the handshake operation: a success (`Ok`) returns a `String` (e.g., a session identifier),
/// and a failure (`Err`) returns an `io::Error`.
///
#[async_trait]
pub trait Handshake {
    /// Performs an asynchronous handshake operation.
    ///
    /// # Parameters
    ///
    /// * `timeout`: An optional timeout for the handshake operation specified in seconds.
    /// If `Some(timeout)`, the operation should complete or fail within this duration.
    /// If `None`, the operation can take indefinitely long.
    ///
    /// # Returns
    ///
    /// A `Result<String, io::Error>` indicating the outcome of the handshake:
    /// - `Ok(String)`: Handshake succeeded, with the `String` representing success details.
    /// - `Err(io::Error)`: Handshake failed due to an I/O error.
    async fn shake(&self, timeout: Option<u32>) -> io::Result<String>;
}
