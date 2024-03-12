mod cli;
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let handshake = cli::Args::parse();

    match handshake.command {
        cli::Command::Connect { node } => {
            println!("Connecting to {}", node);
        }
    }

    Ok(())
}
