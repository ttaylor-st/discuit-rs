use crate::structs::api_requests::*;
use crate::structs::api_responses::*;
use crate::structs::api_types::*;
use crate::structs::internal_types::*;
use reqwest::{Client, ClientBuilder};

/// DiscuitClient represents a client for the Discuit API and
/// provides methods to interact with the API.
#[derive(Debug)]
pub struct DiscuitClient {
    /// The HTTP client.
    client: Client,
    /// The CSRF token.
    csrf_token: String,
    /// The session ID.
    session_id: String,
    /// The base URL of the Discuit instance.
    base_url: String,
    /// The user agent to use for requests.
    /// Defaults to "DiscuitClient".
    user_agent: String,
    /// The User object of the authenticated user.
    /// If the client is not authenticated, this is None.
    user: Option<User>,
    /// `log_level` is the level of logging to use.
    /// Defaults to `LogLevel::Info`.
    log_level: LogLevel,
}

impl DiscuitClient {
    /// Logger is a helper function to log messages based on the log level.
    /// For internal use only.
    fn log(&self, level: LogLevel, message: &str) {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");

        if level >= self.log_level {
            println!("{} [{}] {}", timestamp, level, message);
        }
    }

    /// Create a new DiscuitClient with the given base URL.
    pub fn new(base_url: &str) -> Self {
        let base_url = base_url.trim_end_matches('/');

        let client = ClientBuilder::new()
            .user_agent("DiscuitClient")
            .cookie_store(true)
            .build()
            .unwrap();

        Self {
            client,
            csrf_token: String::new(),
            session_id: String::new(),
            base_url: base_url.to_string(),
            user_agent: "DiscuitClient".to_string(),
            user: None,
            log_level: LogLevel::Debug,
        }
    }

    /// Resets the client to its initial state.
    pub fn reset(&mut self) {
        self.log(LogLevel::Info, "Resetting client ...");
        self.csrf_token = String::new();
        self.session_id = String::new();
        self.user = None;
        self.log(LogLevel::Info, "Client reset.");
    }

    /// Initialize the client by fetching a CSRF token and a session ID.
    /// Returns `InitialResponse`.
    pub async fn initialize(&mut self) -> Result<InitialResponse, reqwest::Error> {
        self.log(LogLevel::Info, "Initializing client ...");
        self.log(
            LogLevel::Info,
            &format!("GET {}/api/_initial", self.base_url),
        );
        let response = self
            .client
            .get(&format!("{}/api/_initial", self.base_url))
            .send()
            .await?;

        let cookies = response.cookies();
        for cookie in cookies {
            if cookie.name() == "csrftoken" {
                self.csrf_token = cookie.value().to_string();
                self.log(LogLevel::Info, &format!("CSRF token: {}", self.csrf_token));
            } else if cookie.name() == "SID" {
                self.session_id = cookie.value().to_string();
                self.log(LogLevel::Info, &format!("Session ID: {}", self.session_id));
            }
        }

        let text = response.text().await?;
        let initial_response: InitialResponse = serde_json::from_str(&text).unwrap();
        self.log(
            LogLevel::Debug,
            &format!("Initial response: {:#?}", initial_response),
        );
        self.log(LogLevel::Info, "Client initialized.");
        Ok(initial_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_initialize() {
        let mut client = DiscuitClient::new("https://discuit.net");
        let response = client.initialize().await;

        println!("{:?}", response);

        assert!(!client.csrf_token.is_empty());
        assert!(!client.session_id.is_empty());
        assert!(response.is_ok());
    }
}
