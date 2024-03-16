# Simple Solana Node Handshake

## Limitations and Caveats

As this is a simple handshake project, there are some limitations on its functionality.

- It doesn't support the usage of proxies.
- It only supports Ip4 addresses.
- It is a simple connect and handshake - not robust enough for more complex ops
- It assumes that you are specifying the address correctly, no probing is done prior to the handshake attempt.
- It assumes that you pass the scheme (http or https) in the host argument.
- The gossip node uses the Solana sdk for key generation - handrolling this would have taken too long.

## Path to usage

### Installation/Build

#### Take the high road

The easiest way to build and test is to use Nix. This not only gives the support for running the project (should you not have Rust installed) but also supplies the Solana toolchain so that a test validator (rpc node) can be spun up locally.

##### Install Nix

Run the following in a terminal:

```bash
sh <(curl -L https://nixos.org/nix/install) --daemon
```

##### Enable flake support

Add the following to `~/.config/nix/nix.conf` or `/etc/nix/nix.conf`:

```
experimental-features = nix-command flakes
```

##### Usage

In order to use the handshake you will need Rust support and a running Solana RPC node. To enable usage of the tool you will need two terminals, one to run the solana-test-validator and another to run the handshake. In both terminals, you will need to run a nix development shell by running the following:

```bash
nix develop --impure
```

In the one of the terminals the Solana test validator needs to be run:

```bash
solana-test-validator
```

#### Take the low road

If you are not using, or don't want to use, Nix; then you will need to install Rust and the Solana toolchain.

##### Installing Rust

###### Download the Rust Installer

- **Windows**: Download the Rust installer from the official Rust website (https://www.rust-lang.org/tools/install). Look for the "rustup-init.exe" file and download it.
- **macOS and Linux**: Open a terminal and run the following command:

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

  This command downloads a script and starts the installation of the rustup tool, which installs Rust.

###### Run the Installer

- **Windows**: Run the "rustup-init.exe" file and follow the on-screen instructions.
- **macOS and Linux**: After running the curl command in the terminal, the rustup script will start automatically. Follow the instructions provided in the terminal.

###### Configure Your Path

Rust installation is almost complete. Now, you need to ensure that your system's PATH environment variable includes Rust.

- **Windows**: The installer should automatically add Rust to your system PATH. Restart your terminal or computer to ensure the changes take effect.
- **macOS and Linux**: If you're using a Unix-like OS, the rustup script suggests running a command to add Rust to your PATH. Typically, it involves modifying your shell profile file (like `.bash_profile`, `.zshrc`, etc.). If rustup does not do this automatically, you can add the following line to your profile:

  ```bash
  export PATH="$HOME/.cargo/bin:$PATH"
  ```

  After adding the line, reload your profile (e.g., `source ~/.bash_profile`) or restart your terminal.

###### Verify Installation

To confirm that Rust is installed correctly, open a new terminal and run:

```bash
rustc --version
```

If the installation was successful, you should see the version of Rust printed to the console.

##### Install the Solana toolchain

###### Run the install script

```bash
sh -c "$(curl -sSfL https://release.solana.com/v1.10.32/install)"
```

###### Configure your Path

```bash
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
```

###### Verfiy the installation

```bash
solana --version
```

###### Run the Solana test validator

```bash
solana-test-validator
```

You should see a terminal output like the following:

```bash
Version: 1.16.26
Shred Version: 43529
Gossip Address: 127.0.0.1:1024
TPU Address: 27.0.0.1:1027
JSON RPC URL: http://127.0.0.1:8899
WebSocket PubSub URL: ws://127.0.0.1:8900
```

This output will show the connection urls and you can move on to running the tool.

### Building the tool

Build the tool using cargo:

```bash
cargo build --release
```

The handshake binary will then be located at `/target/release/handshake`.

### Running the tool

#### Handshake with the JSON RPC endpoint

You can run the handshake with the Solana test validator with the following command in a terminal:

```bash
./target/release/handshake connect-rpc --address "127.0.0.1:8899"
```

##### Connecting to a HTTPS endpoint

If you are connecting to a non-local node that is HTTPS you can use the secure flag to connect:

```bash
./target/release/handshake connect-rpc --address "127.0.0.1:8899" --secure
```

#### Handshake with the Websocket PubSub endpoint

You can run the handshake with the Solana test validator with the following command in a terminal:

```bash
./target/release/handshake connect-rpc-with-websocket --address "127.0.0.1:8900"
```

##### Connecting to a WSS endpoint

If you are connecting to a non-local node that is WSS you can use the secure flag to connect:

```bash
./target/release/handshake connect-rpc-with-websocket --address "127.0.0.1:8900" --secure
```
