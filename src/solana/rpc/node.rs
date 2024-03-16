// Main module for RPC functionality with support for multiple transport types.
// This includes the definitions for the RpcNode struct, transport selection, and handshake mechanisms.
use crate::solana::{
    node::Handshake,
    transport::{tcp::Tcp, ws::Ws, ChooseTransport, Transport},
    TransportType,
}; // Import necessary traits and structures for handshake and transport.
use async_trait::async_trait; // Enables async trait methods, crucial for async network operations.
use serde::{Deserialize, Serialize}; // Allows for easy serialization and deserialization of data structures.
use std::{fmt, io, net::SocketAddr}; // Standard library imports for networking and display formatting.

// Struct defining an RPC node, including its remote address and transport type for connectivity.
pub struct RpcNode {
    pub remote: SocketAddr,            // Socket address of the remote endpoint.
    pub transport_type: TransportType, // Enum specifying the type of transport to use.
}

// Response structure expected from an RPC handshake, defining how to deserialize the JSON response.
#[derive(Serialize, Deserialize, Debug)]
pub struct RpcHandshakeResponse {
    #[serde(rename = "solana-core")] // Maps "solana-core" field in JSON to solana_core attribute.
    solana_core: String, // The version of the solana-core software running on the node.
    #[serde(rename = "feature-set")]
    // Optional field indicating the feature set supported by the node.
    feature_set: Option<u64>,
}

// Request structure for initiating an RPC handshake, specifying the expected JSON structure.
#[derive(Serialize, Deserialize, Debug)]
pub struct RpcHandshakeRequest {
    #[serde(rename = "jsonrpc")] // Specifies the JSON RPC version being used.
    json_rpc: String,
    id: u64,        // Unique identifier for the request.
    method: String, // The RPC method being called, in this case, to get the node version.
}

impl RpcNode {
    // Constructor for RpcNode, taking a socket address and transport type.
    pub fn new(remote: SocketAddr, transport_type: TransportType) -> Self {
        RpcNode {
            remote,
            transport_type,
        }
    }

    // Generates the JSON payload for the handshake request, conforming to the RPC's expected format.
    pub fn get_handshake_payload(&self) -> serde_json::Value {
        serde_json::json!(RpcHandshakeRequest {
            json_rpc: "2.0".to_string(),      // Using JSON RPC version 2.0.
            id: 1,                            // Example request ID.
            method: "getVersion".to_string(), // Requesting the version of the solana-core.
        })
    }
}

// Implement the Display trait for RpcNode for easy logging and debugging.
impl fmt::Display for RpcNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RpcNode({})", self.remote) // Custom display format showing the remote address.
    }
}

// Chooses the appropriate transport mechanism based on the transport_type attribute.
impl ChooseTransport for RpcNode {
    fn get_transport(&self) -> Box<dyn Transport> {
        match self.transport_type {
            TransportType::Ws => Box::new(Ws::new(self.remote.to_string(), false)), // WebSocket transport.
            TransportType::Wss => Box::new(Ws::new(self.remote.to_string(), true)), // Secure WebSocket transport.
            TransportType::Tls => Box::new(Tcp::new(self.remote.to_string(), true)), // TLS transport over TCP.
            TransportType::Tcp => Box::new(Tcp::new(self.remote.to_string(), false)), // Plain TCP transport.
        }
    }
}

// Async trait for performing the handshake operation, utilizing the selected transport to connect to the remote node.
#[async_trait]
impl Handshake for RpcNode {
    async fn shake(&self, timeout: Option<u32>) -> io::Result<String> {
        let transport = self.get_transport(); // Dynamically selects the appropriate transport.
        let payload = self.get_handshake_payload(); // Constructs the handshake payload.
                                                    // Initiates the handshake, sending the payload and waiting for a response.
        let raw_response = transport.connect_and_send(timeout, payload).await?;
        // Processes the raw response to extract the JSON payload.
        let json_start = raw_response.find("\r\n\r\n").unwrap_or(0) + 4;
        let json_str = &raw_response[json_start..];

        Ok(json_str.to_owned()) // Returns the JSON string extracted from the response.
    }
}

#[cfg(test)]
mod tests {
    // Test suite for RpcNode functionality, ensuring correct creation and payload generation.
    use {super::*, std::str::FromStr};

    // Verifies that an RpcNode can be correctly instantiated with specified parameters.
    #[test]
    fn test_rpc_node_creation() {
        let address = "127.0.0.1:8080";
        let addr = SocketAddr::from_str(address).unwrap();
        let node = RpcNode::new(addr, TransportType::Tcp); // Testing with TCP transport type.

        assert_eq!(node.remote, addr); // Checks that the remote address matches.
        assert_eq!(node.transport_type, TransportType::Tcp); // Ensures the transport type is correctly set.
    }

    // Tests the generation of a handshake payload, verifying it matches expected values.
    #[test]
    fn test_handshake_payload() {
        let address = "127.0.0.1:8080";
        let addr = SocketAddr::from_str(address).unwrap();
        let node = RpcNode::new(addr, TransportType::Ws); // WebSocket as the transport type for this test.

        let payload = node.get_handshake_payload(); // Generate the handshake payload.
                                                    // Expected JSON structure of the payload.
        let expected_payload = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getVersion"
        });

        assert_eq!(payload, expected_payload); // Compare the generated payload to the expected payload.
    }
}
