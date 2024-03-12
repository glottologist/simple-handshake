use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about = "A simple node  handshake")]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Connect {
        #[clap(short, long)]
        node: String,
    },
}
