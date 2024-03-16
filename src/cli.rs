// Import necessary crates for network operations, error handling, and CLI parsing.
// Clap is used for parsing command-line arguments, std for standard operations, especially related to I/O and networking.
use {
    clap::{ArgAction, Args, Parser, Subcommand},
    std::{
        io::{Error, ErrorKind, Result},
        net::{SocketAddr, ToSocketAddrs},
    },
};

// Attempts to resolve a given target string (e.g., "api.devnet.solana.com") into a `SocketAddr`.
// This will fail if the DNS lookup fails, indicating the URL appears valid but lacks a DNS entry.
fn resolve_target(target: &str) -> Result<SocketAddr> {
    // Convert the target URL into a socket address, selecting the first resolved address if successful.
    // If no address is resolved, return a custom error indicating the destination could not be found.
    let socketaddr = target.to_socket_addrs()?.next().ok_or_else(|| {
        Error::new(
            ErrorKind::AddrNotAvailable,
            format!("Could not find destination {target}"),
        )
    })?;
    Ok(socketaddr)
}

// Defines the command-line interface structure for the application, utilizing Clap for argument parsing.
// The application provides a simple handshake mechanism with Solana RPC nodes, supporting both TCP and WebSocket connections.
#[derive(Parser)]
#[clap(
    author,
    version,
    about = "A simple Solana node handshake",
    long_about = "Provides a simple handshake with a Solana RPC node using both TCP and Websockets."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command, // Enumerates the different operations supported by the CLI, parsed as subcommands.
}

// Contains arguments specific to the Node operation, including the target address and a security flag for secure connections.
#[derive(Args)]
pub struct NodeArgs {
    // Address of the Solana node to connect to, parsed using the `resolve_target` function.
    // Help message guides users to omit the URL scheme for the address, offering advice on secure connection flags.
    #[arg(short, long, value_parser = resolve_target, help = "Supply the address without the scheme, i.e. 'api.testnet.solana.com'. Use the '--secure' flag for secure connections.")]
    pub address: SocketAddr,

    // Flag indicating whether a secure connection should be established, parsed as a boolean value.
    #[arg(action = ArgAction::SetTrue, short, long = "secure", help = "Indicates a secure connection is required.")]
    pub secure: bool,
}

// Enumerates possible subcommands available in the CLI, allowing users to specify the type of connection to establish.
#[derive(Subcommand)]
pub enum Command {
    // Subcommand for establishing a TCP connection to a Solana RPC node, accepting NodeArgs for connection parameters.
    #[command(aliases = ["crp"])]
    ConnectRpc(NodeArgs),
    // Subcommand for establishing a WebSocket connection to a Solana RPC node, also accepting NodeArgs.
    #[command(aliases = ["cws"])]
    ConnectRpcWithWebsocket(NodeArgs),
}

#[cfg(test)]
mod tests {
    // Includes tests for the `resolve_target` function and property-based tests for handling domain resolution.
    use {
        super::*,
        proptest::{
            prelude::{Just, ProptestConfig, Strategy},
            prop_oneof, proptest,
        },
        test_case::test_case,
    };

    // Defines test cases for the `resolve_target` function, covering both expected successes and a failure scenario.
    #[test_case("127.0.0.1:1024"; "when url is loopback")]
    #[test_case("localhost:1024"; "when url is localhost")]
    #[test_case("api.devnet.solana.com:1024"; "when url is devnet")]
    #[test_case("api.testnet.solana.com:1024"; "when url is testnet")]
    #[test_case("api.mainnet-beta.solana.com:1024"; "when url is mainnet")]
    #[test_case("localhost:0"; "when url has 0 port")]
    // Tests `resolve_target` with various URLs, expecting successful resolution.
    fn test_resolve_target(url: &str) {
        let target = resolve_target(url);
        assert!(
            target.is_ok(),
            "Expected the target to be resolved successfully."
        );
    }

    #[test_case("localhost:65536"; "when url port is higher than maximum port")]
    // Tests `resolve_target` with an invalid port, expecting failure.
    fn test_resolve_target_failures(url: &str) {
        let target = resolve_target(url);
        assert!(
            target.is_err(),
            "Expected the target resolution to fail due to an invalid port."
        );
    }

    // Strategy for generating syntactically valid but non-existent domain names for testing failure scenarios in DNS resolution.
    fn invalid_domain() -> impl Strategy<Value = String> {
        // Constructs domain names using random characters and common suffixes, excluding transport protocol prefixes.
        let scheme = prop_oneof![Just("http://"), Just("https://")];
        let www = prop_oneof![Just("www."), Just("")];
        let domain = "[a-z]{5,10}";
        let suffix = prop_oneof![
            Just(".com"),
            Just(".net"),
            Just(".io"),
            Just(".xyz"),
            Just(".co.uk")
        ];

        // Maps the components to form a complete URL string.
        (scheme, www, domain, suffix).prop_map(|(scheme, www, domain, suffix)| {
            format!("{}{}{}{}", scheme, www, domain, suffix)
        })
    }

    // Strategy for generating valid port numbers within the acceptable range.
    fn valid_port_strategy() -> impl Strategy<Value = u32> {
        // Includes the entire range of valid ports except for reserved ports.
        prop_oneof![1024u32..65535u32,]
    }

    // Property-based test to examine the behavior of `resolve_target` using constructed domains and valid ports.
    // The test expects failure, assuming the generated domains do not resolve to actual addresses.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]
        #[test]
        fn test_resolve_target_with_good_ports_prop(domain in invalid_domain(), port in valid_port_strategy()) {
            // Constructs a URL combining the domain and port, expecting resolution to fail.
            let url = format!("{}:{}", domain, port);
            let target = resolve_target(&url);
            assert!(target.is_err(), "Expected an error when resolving artificially constructed domain: {}", url);
        }
    }
}
