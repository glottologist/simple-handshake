// Imports the `async_trait` macro for enabling asynchronous methods within traits,
// and the standard `io` module for input/output operations, including network communication and error handling.
use {async_trait::async_trait, std::io};

// Module declarations for TCP and WebSocket implementations,
// allowing for specific transport protocol functionality to be encapsulated within these modules.
pub mod tcp;
pub mod ws;

// Define the `Transport` trait for asynchronous network communication.
// This trait provides a generic interface for sending data across a network and receiving a response.
#[async_trait]
pub trait Transport: Send + Sync {
    // Ensure the trait objects are safe to send and share across threads.
    // An asynchronous method that attempts to connect to a remote endpoint, send a payload,
    // and await a response. It accepts an optional timeout and a JSON payload as parameters.
    async fn connect_and_send(
        &self,
        timeout: Option<u32>,       // Optional timeout in seconds.
        payload: serde_json::Value, // The payload to be sent, encapsulated as JSON.
    ) -> io::Result<String>; // Returns an `io::Result` encapsulating the response as a `String` or an error.
}

// Define the `ChooseTransport` trait for selecting the appropriate transport mechanism at runtime.
pub trait ChooseTransport {
    // A method that returns an instance of a type implementing the `Transport` trait, encapsulated in a `Box<dyn Transport>`.
    // This allows for dynamic dispatch to different transport implementations.
    fn get_transport(&self) -> Box<dyn Transport>
    where
        Self: Sized; // Ensures this method can only be called on types that are sized, allowing for self-references.
}

