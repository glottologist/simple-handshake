// This is the main module for the RPC functionality.
// It may contain the core logic for performing remote procedure calls,
// handling requests, and responses.
pub mod rpc;

// The `transport` module is responsible for the underlying communication
// mechanisms. It likely abstracts over different transport layers
// such as TCP, TLS, WebSocket, etc., providing a unified interface for the RPC system.
pub mod transport;

// The `node` module could represent entities within the RPC system,
// such as client and server nodes. It might contain implementations
// for initiating connections, managing state, and sending or receiving messages.
pub mod node;

// `TransportType` enum defines the supported types of transport protocols
// for the RPC system. Each variant represents a different protocol
// that can be used for communication between nodes.
#[derive(Debug, PartialEq)]
pub enum TransportType {
    // Represents plain TCP transport, a standard, low-level protocol
    // for network communication without encryption.
    Tcp,

    // Represents Transport Layer Security (TLS) transport, building on TCP
    // to provide encrypted communication, ensuring privacy and data integrity.
    Tls,

    // Represents WebSocket (Ws) transport, a protocol providing full-duplex
    // communication channels over a single TCP connection, often used for
    // real-time data transfer in web applications.
    Ws,

    // Represents Secure WebSocket (Wss) transport, an extension of WebSocket
    // that runs over TLS for secure communication.
    Wss,
}
