mod cli;
use clap::Parser;
use cli::{Cli, Command};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let handshake = Cli::parse();

    match handshake.command {
        Command::ConnectRpc(node) => {
            println!("Connecting to RPC {}:{}", node.host, node.port.to_string());
        }
        Command::ConnectGossip(node) => {
            println!(
                "Connecting to Gossip {}:{}",
                node.host,
                node.port.to_string()
            );
        }
    }

    Ok(())
}
