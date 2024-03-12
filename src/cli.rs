use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about = "A simple node  handshake")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Args)]
pub struct NodeArgs {
    pub host: String,
    #[arg( value_parser = clap::value_parser!(u16))]
    pub port: u16,
}

#[derive(Subcommand)]
pub enum Command {
    ConnectRpc(NodeArgs),
    ConnectGossip(NodeArgs),
}
