// Import the Transport trait from the parent module for polymorphic use across different transport implementations.
// Import the async_trait macro to enable async function definitions in traits, which is not natively supported in Rust.
use tokio::net::TcpStream; // Import the TcpStream struct from the tokio asynchronous runtime for handling TCP operations.
// Grouped import for clarity and organization.
use {
    super::Transport, // Import the Transport trait for implementing custom transport logic.
    async_trait::async_trait, // Import async_trait for asynchronous trait methods.
    tracing::info,    // Import logging macros for structured error and informational logging.
};
// Grouped import for TLS configuration and asynchronous IO operations.
use {
    rustls::ServerName, // Import ServerName for DNS name validation in TLS connections.
    std::{
        io::{self, Error, ErrorKind}, // Import standard IO types for error handling.
        sync::Arc,                    // Import Arc for thread-safe reference counting.
    },
    tokio::io::{AsyncReadExt, AsyncWriteExt}, // Import extensions for asynchronous reading and writing.
    tokio_rustls::{
        rustls::{ClientConfig, RootCertStore}, // Import TLS types for configuration.
        TlsConnector, // Import TlsConnector for initiating TLS connections.
    },
    webpki_roots::TLS_SERVER_ROOTS, // Import TLS server root certificates for trusted CA validation.
};

// Define a constant for the default TCP timeout duration in seconds.
const TCP_TIMEOUT_SECONDS: u32 = 60;

// Define the Tcp struct representing a TCP transport layer with a remote address and security preference.
pub struct Tcp {
    remote: String,  // The remote server's address as a string.
    is_secure: bool, // Flag indicating whether to use secure WebSocket (WSS) or not.
}

// Implementation block for Tcp.
impl Tcp {
    // Constructs a new Tcp instance with the specified remote address and security preference.
    pub fn new(remote: String, is_secure: bool) -> Self {
        Tcp { remote, is_secure }
    }
}

// Creates a TLS configuration for secure TCP connections.
fn create_tls_config() -> Result<ClientConfig, Box<dyn std::error::Error>> {
    let mut root_store = RootCertStore::empty(); // Initialize an empty RootCertStore.

    // Add server trust anchors from the webpki_roots crate to the root store.
    root_store.add_server_trust_anchors(TLS_SERVER_ROOTS.0.iter().map(|ta| {
        rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));

    // Create a ClientConfig with the populated root store for TLS connections.
    let config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth(); // No client authentication for simplicity.

    Ok(config)
}

// Asynchronously connects to a secure remote server, sends a request, and receives the response.
async fn connect_and_send_secure(
    remote: &str,          // Remote host address.
    _timeout: Option<u32>, // Optional TCP operation timeout in seconds.
    req: String,           // Request payload to send.
) -> io::Result<String> {
    // Validate and parse the remote server's DNS name.
    let dns_name = ServerName::try_from(remote)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid DNS name"))?;

    // Create a TLS configuration or return an error.
    let config = match create_tls_config() {
        Ok(c) => c,
        Err(e) => {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Unable to create TLS config: {}", e),
            ))
        }
    };
    let connector = TlsConnector::from(Arc::new(config)); // Wrap the config in an Arc for thread safety.
    let stream = TcpStream::connect(&remote).await?; // Connect to the remote server asynchronously.
    let mut stream = connector.connect(dns_name, stream).await?; // Establish a TLS connection.

    // Write the request to the TLS stream and read the response.
    stream.write_all(req.as_bytes()).await?;
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await?;

    // Convert the response buffer to a UTF-8 string.
    let response = String::from_utf8_lossy(&buf).to_string();
    Ok(response)
}

// Asynchronously connects to an insecure remote server, sends a request, and receives the response.
async fn connect_and_send_insecure(
    remote: &str,         // Remote host address.
    timeout: Option<u32>, // Optional TCP operation timeout in seconds.
    req: String,          // Request payload to send.
) -> io::Result<String> {
    let stream = TcpStream::connect(remote).await?; // Connect to the remote server asynchronously.
    let _ = stream.set_ttl(timeout.unwrap_or(TCP_TIMEOUT_SECONDS)); // Set the TTL for TCP packets.
    stream.writable().await?; // Wait until the stream is writable.

    // Send the request payload.
    info!("Sent message payload {}", &req);
    let _ = stream.try_write(req.as_bytes());

    // Read the response into a buffer.
    let mut buf = vec![0; 1024];
    loop {
        stream.readable().await?;
        match stream.try_read(&mut buf) {
            Ok(n) => {
                buf.truncate(n); // Truncate the buffer to the size of the data read.
                info!("Received message of length {}", &n);
                break;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue, // Continue reading if the operation would block.
            Err(e) => return Err(e),
        }
    }

    // Convert the response buffer to a UTF-8 string and return it.
    let response: String = String::from_utf8_lossy(&buf).to_string();
    info!("Received message was {}", &response);
    Ok(response.trim_end().to_owned())
}

// Implement the Transport trait for the Tcp struct, allowing for asynchronous connection and data transfer.
#[async_trait]
impl Transport for Tcp {
    async fn connect_and_send(
        &self,                      // Reference to self for method invocation on an instance.
        timeout: Option<u32>,       // Optional TCP operation timeout in seconds.
        payload: serde_json::Value, // JSON payload to be sent.
    ) -> io::Result<String> {
        // Format the HTTP request with JSON content.
        let json_header = "Content-Type: application/json";
        let http_header = format!(
            "POST / HTTP/1.1\r\nHost: {}\r\n{}\r\nConnection: close\r\nContent-Length: {}\r\n\r\n",
            &self.remote,
            json_header,
            payload.to_string().as_bytes().len()
        );
        let req = format!("{}{}\r\n", http_header, payload);

        // Log the attempt to connect to the remote endpoint.
        info!("Connected to remote tcp endpoint {}", &self.remote);

        // Choose between secure and insecure connections based on the is_secure flag.
        if self.is_secure {
            connect_and_send_secure(&self.remote, timeout, req).await
        } else {
            connect_and_send_insecure(&self.remote, timeout, req).await
        }
    }
}

#[cfg(test)]
mod tests {
    use {super::*, proptest::prelude::*};

    // Property-based tests to ensure that URL formatting does not cause panics.
    proptest! {
        #[test]
        fn test_url_formatting_does_not_panic(remote in "[a-zA-Z0-9]+") {
            let result = std::panic::catch_unwind(|| {
               let _ = Tcp::new(remote, false).connect_and_send(None, serde_json::json!({}));
            });
            assert!(result.is_ok());
        }
    }
}
