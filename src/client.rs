//! # Typesense asynchronous client.
use std::time::Duration;

/// An asynchronous Typesense client.
pub struct Client<'a> {
    api_key: &'a str,
    connection_timeout: Duration,
    healthcheck_interval: Duration,
    nodes: Vec<&'a str>,
    num_retries: usize,
    retry_interval: Duration,
}

/// Builder for the Typesense `Client`.
///
/// This type can be used to construct an instance of `Client` through a
/// builder-like pattern.
pub struct ClientBuilder<'a> {
    api_key: &'a str,
    connection_timeout: Duration,
    healthcheck_interval: Duration,
    nodes: Vec<&'a str>,
    nearest_node: Option<&'a str>,
    num_retries: usize,
    retry_interval: Duration,
}

impl<'a> Default for ClientBuilder<'a> {
    fn default() -> Self {
        Self {
            api_key: "",
            connection_timeout: Duration::from_secs(3),
            healthcheck_interval: Duration::from_secs(60),
            nodes: vec![],
            nearest_node: None,
            num_retries: 3,
            retry_interval: Duration::from_secs(3),
        }
    }
}

impl<'a> ClientBuilder<'a> {
    /// Set API Key.
    pub fn api_key(mut self, key: &'a str) -> Self {
        self.api_key = key;
        self
    }
    /// Create `Client` instance
    pub fn build(self) -> Client<'a> {
        Client {
            api_key: self.api_key,
            connection_timeout: self.connection_timeout,
            healthcheck_interval: self.healthcheck_interval,
            nodes: self.nodes,
            num_retries: self.num_retries,
            retry_interval: self.retry_interval,
        }
    }

    /// Set connection timeout.
    ///
    /// Default value is 3 seconds.
    pub fn connection_timeout(mut self, timeout: Duration) -> Self {
        self.connection_timeout = timeout;

        self
    }

    /// Set connection timeout.
    ///
    /// Default value is 3.
    pub fn num_retries(mut self, retries: usize) -> Self {
        self.num_retries = retries;

        self
    }

    /// Include node.
    pub fn node(mut self, node: &'a str) -> Self {
        self.nodes.push(node);

        self
    }

    /// Set nearest node.
    pub fn nearest_node(mut self, node: &'a str) -> Self {
        self.nearest_node = Some(node);

        self
    }

    /// Set retry interval.
    ///
    /// Default value is 3 seconds.
    pub fn retry_interval(mut self, interval: Duration) -> Self {
        self.retry_interval = interval;

        self
    }

    /// Set healthcheck interval.
    ///
    /// Default value is 3 seconds.
    pub fn healthcheck_interval(mut self, interval: Duration) -> Self {
        self.healthcheck_interval = interval;

        self
    }
}
