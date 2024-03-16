// Import necessary modules and crates for CLI handling, networking, and logging.
use clap::Parser; //Use clap parser
use cli::{Cli, Command}; // Assuming these are defined in a local `cli` module for parsing CLI commands.
use handshake::solana::node::Handshake; // Interface for performing handshake operations.
use handshake::solana::rpc::node::RpcNode; // RPC node handling functionalities.
use handshake::solana::TransportType; // Enum for different transport types (TCP, TLS, WS, WSS).
use tracing::info; // Import the `info` macro for logging informational messages.
mod cli; // Import the CLI module which defines the `Cli` and `Command` structures.

// Async entrypoint
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize `tracing` for application-wide logging.
    // This setup uses default settings for simplicity, suitable for most applications.
    tracing_subscriber::fmt::init();

    // Parse command-line arguments into the `Cli` struct, leveraging Clap's derive macros for parsing.
    let cli = Cli::parse();

    // Handle the parsed command using pattern matching to decide the flow based on user input.
    match cli.command {
        // If the command is to connect via RPC, handle accordingly.
        Command::ConnectRpc(node) => {
            // Determine whether to use secure transport (TLS) or plain TCP based on the `secure` flag.
            let trans_type = match node.secure {
                true => TransportType::Tls,
                false => TransportType::Tcp,
            };

            // Instantiate an RPC node with the provided address and determined transport type.
            let rpc_node = RpcNode::new(node.address, trans_type);

            // Log the connection attempt.
            info!("Connecting to {}", rpc_node);

            // Attempt to perform a handshake with the RPC node, awaiting the asynchronous operation.
            let response = rpc_node.shake(None).await?;

            // Log the response from the handshake operation.
            info!("Handshake response was {:?}", response);
        }
        // If the command is to connect via WebSocket, the process is similar but with WebSocket protocols.
        Command::ConnectRpcWithWebsocket(node) => {
            // Choose between secure WebSocket (WSS) or plain WebSocket (WS) based on the `secure` flag.
            let trans_type = match node.secure {
                true => TransportType::Wss,
                false => TransportType::Ws,
            };

            // As before, instantiate an RPC node for WebSocket connection and log the attempt.
            let rpc_node = RpcNode::new(node.address, trans_type);
            info!("Connecting to {}", rpc_node);

            // Perform the handshake over WebSocket, logging the response.
            let response = rpc_node.shake(None).await?;
            info!("Handshake response was {:?}", response);
        }
    }

    // If the command execution succeeds, return Ok.
    Ok(())
}
