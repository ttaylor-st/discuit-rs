use crate::structs::api_requests::*;
use crate::structs::api_responses::*;
use crate::structs::api_types::*;
use crate::structs::internal_types::*;
use reqwest::{Client, ClientBuilder};
use std::collections::HashMap;
use url::Url;

/// DiscuitClient represents a client for the Discuit API and
/// provides methods to interact with the API.
#[derive(Debug)]
pub struct DiscuitClient {
    /// The HTTP client.
    pub client: Client,
    /// The CSRF token.
    pub csrf_token: String,
    /// The session ID.
    pub session_id: String,
    /// The base URL of the Discuit instance.
    pub base_url: String,
    /// The user agent to use for requests.
    /// Defaults to "DiscuitClient".
    pub user_agent: String,
    /// The User object of the authenticated user.
    /// If the client is not authenticated, this is None.
    pub user: Option<User>,
    /// `log_level` is the level of logging to use.
    /// Defaults to `LogLevel::Info`.
    pub log_level: LogLevel,
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

    /// Log in to the Discuit instance with the given username and password.
    pub async fn login(
        &mut self,
        username: &str,
        password: &str,
    ) -> Result<UserResponse, reqwest::Error> {
        self.log(LogLevel::Info, "Logging in ...");
        self.log(
            LogLevel::Info,
            &format!("POST {}/api/_login", self.base_url),
        );
        let response = self
            .client
            .post(&format!("{}/api/_login", self.base_url))
            .header("X-Csrf-Token", &self.csrf_token)
            .header(
                "Cookie",
                &format!("csrftoken={}; SID={}", self.csrf_token, self.session_id),
            )
            .json(&LoginRequest {
                username: username.to_string(),
                password: password.to_string(),
            })
            .send()
            .await?;

        let text = response.text().await?;
        let login_response: UserResponse = serde_json::from_str(&text).unwrap();
        self.log(
            LogLevel::Debug,
            &format!("Login response: {:#?}", login_response),
        );
        self.log(LogLevel::Info, "Logged in.");
        self.user = match &login_response {
            UserResponse::Error(_) => None,
            UserResponse::User(user) => Some(user.clone()),
        };

        Ok(login_response)
    }

    /// Log out of the Discuit instance.
    /// Returns either `()` or a `reqwest::Error`.
    pub async fn logout(&mut self) -> Result<(), reqwest::Error> {
        self.log(LogLevel::Info, "Logging out ...");
        if self.user.is_none() {
            self.log(LogLevel::Info, "Not logged in.");
            return Ok(());
        }

        self.log(
            LogLevel::Info,
            &format!("POST {}/api/_login?action=logout", self.base_url),
        );
        let response = self
            .client
            .post(&format!("{}/api/_login?action=logout", self.base_url))
            .header("X-Csrf-Token", &self.csrf_token)
            .header(
                "Cookie",
                &format!("csrftoken={}; SID={}", self.csrf_token, self.session_id),
            )
            .send()
            .await?;

        self.log(LogLevel::Info, "Logged out.");
        self.log(
            LogLevel::Debug,
            &format!("Logout response: {:#?}", response),
        );
        self.reset();
        Ok(())
    }

    /// Fetch the current user from the Discuit instance.
    /// Returns either `User` or a `reqwest::Error`.
    pub async fn get_user(&mut self) -> Result<User, reqwest::Error> {
        self.log(LogLevel::Info, "Fetching user ...");
        self.log(LogLevel::Info, &format!("GET {}/api/_user", self.base_url));
        let response = self
            .client
            .get(&format!("{}/api/_user", self.base_url))
            .header("X-Csrf-Token", &self.csrf_token)
            .header(
                "Cookie",
                &format!("csrftoken={}; SID={}", self.csrf_token, self.session_id),
            )
            .send()
            .await?;

        let text = response.text().await?;
        let user: User = serde_json::from_str(&text).unwrap();
        self.log(LogLevel::Debug, &format!("User: {:#?}", user));
        self.log(LogLevel::Info, "User fetched.");
        Ok(user)
    }

    /// Fetch another user from the Discuit instance.
    /// Returns UserResponse
    pub async fn get_user_by_username(
        &mut self,
        username: &str,
    ) -> Result<UserResponse, reqwest::Error> {
        self.log(LogLevel::Info, "Fetching user by username ...");
        self.log(
            LogLevel::Info,
            &format!("GET {}/api/users/{}", self.base_url, username),
        );
        let response = self
            .client
            .get(&format!("{}/api/users/{}", self.base_url, username))
            .header("X-Csrf-Token", &self.csrf_token)
            .header(
                "Cookie",
                &format!("csrftoken={}; SID={}", self.csrf_token, self.session_id),
            )
            .send()
            .await?;

        let text = response.text().await?;
        let user_response: UserResponse = serde_json::from_str(&text).unwrap();
        self.log(
            LogLevel::Debug,
            &format!("User by username: {:#?}", user_response),
        );
        self.log(LogLevel::Info, "User by username fetched.");
        Ok(user_response)
    }

    /// Fetch a user's feed.
    pub async fn get_feed(&mut self, username: &str) -> Result<FeedResponse, reqwest::Error> {
        self.log(LogLevel::Info, "Fetching feed ...");
        self.log(
            LogLevel::Info,
            &format!("GET {}/api/users/{}/feed", self.base_url, username),
        );
        let response = self
            .client
            .get(&format!("{}/api/users/{}/feed", self.base_url, username))
            .header("X-Csrf-Token", &self.csrf_token)
            .header(
                "Cookie",
                &format!("csrftoken={}; SID={}", self.csrf_token, self.session_id),
            )
            .send()
            .await
            .unwrap();

        let text = response.text().await.unwrap();
        let feed: FeedResponse = serde_json::from_str(&text).unwrap();
        self.log(LogLevel::Debug, &format!("Feed: {:#?}", feed));
        self.log(LogLevel::Info, "Feed fetched.");

        Ok(feed)
    }

    /// Fetch community or sitewide posts with optional query parameters.
    ///
    /// # Example
    ///
    /// ```
    /// # use discuit_rs::client::*;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let base_url = "https://discuit.net";
    /// let mut client = DiscuitClient::new(base_url);
    ///
    /// // Fetch all posts
    /// let site_posts = client.get_posts(None, None).await.unwrap();
    /// // Fetch posts from the "general" community
    /// let community_posts = client.get_posts(None, Some("general")).await.unwrap();
    /// let sorted_posts = client.get_posts(Some("new"), None).await.unwrap();
    /// # }
    /// ```
    pub async fn get_posts(
        &mut self,
        sort: Option<&str>,
        community: Option<&str>,
    ) -> Result<PostFeedResponse, Box<dyn std::error::Error>> {
        self.log(LogLevel::Info, "Fetching posts ...");

        let mut url =
            Url::parse(&format!("{}/api/posts", self.base_url)).expect("Failed to parse base URL");

        let mut query_params = HashMap::new();
        if let Some(sort) = sort {
            query_params.insert("sort", sort);
        } else {
            query_params.insert("sort", "hot");
        }
        if let Some(community) = community {
            query_params.insert("community", community);
        }

        url.query_pairs_mut().extend_pairs(query_params.into_iter());

        self.log(LogLevel::Info, &format!("GET {}", url));

        let response = self
            .client
            .get(url)
            .header("X-Csrf-Token", &self.csrf_token)
            .header(
                "Cookie",
                &format!("csrftoken={}; SID={}", self.csrf_token, self.session_id),
            )
            .send()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let posts: PostFeedResponse = serde_json::from_str(&text).unwrap();
            self.log(LogLevel::Debug, &format!("Posts: {:#?}", posts));
            self.log(LogLevel::Info, "Posts fetched.");
            Ok(posts)
        } else {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            self.log(
                LogLevel::Info,
                &format!("Failed to fetch posts: {} - {}", status, error_text),
            );
            Err(format!("Failed to fetch posts: {} - {}", status, error_text).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::random;

    fn get_env_var(name: &str) -> Result<String, String> {
        std::env::var(name).map_err(|_| format!("Environment variable {} is not set", name))
    }

    #[tokio::test]
    async fn test_initialize() {
        let mut client = DiscuitClient::new("https://discuit.net");
        let response = client.initialize().await;

        println!("{:?}", response);

        assert!(!client.csrf_token.is_empty());
        assert!(!client.session_id.is_empty());
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_login() {
        let mut client = DiscuitClient::new("https://discuit.net");
        client.initialize().await.unwrap();

        let username =
            get_env_var("DISCUIT_USERNAME").expect("DISCUIT_USERNAME must be set for this test");
        let password =
            get_env_var("DISCUIT_PASSWORD").expect("DISCUIT_PASSWORD must be set for this test");
        client.login(&username, &password).await.unwrap();
        client.logout().await.unwrap();

        // Ensure that the client is logged out
        assert!(client.user.is_none());
        assert!(client.csrf_token.is_empty());
        assert!(client.session_id.is_empty());
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut client = DiscuitClient::new("https://discuit.net");
        client.initialize().await.unwrap();

        let username =
            get_env_var("DISCUIT_USERNAME").expect("DISCUIT_USERNAME must be set for this test");
        let password =
            get_env_var("DISCUIT_PASSWORD").expect("DISCUIT_PASSWORD must be set for this test");
        client.login(&username, &password).await.unwrap();
        let user = client.get_user().await.unwrap();
        client.logout().await.unwrap();

        // Ensure that the client is logged out
        assert!(client.user.is_none());
        assert!(client.csrf_token.is_empty());
        assert!(client.session_id.is_empty());

        // Ensure that the user is fetched
        assert_eq!(user.username, username);
    }

    #[tokio::test]
    async fn test_get_user_by_username() {
        let mut client = DiscuitClient::new("https://discuit.net");
        client.initialize().await.unwrap();

        let username =
            get_env_var("DISCUIT_USERNAME").expect("DISCUIT_USERNAME must be set for this test");
        let user = client.get_user_by_username(&username).await.unwrap();

        match user {
            UserResponse::User(user) => {
                assert_eq!(user.username, username);
            }
            _ => panic!("Expected UserResponse::User"),
        }
    }

    #[tokio::test]
    async fn test_get_user_by_username_nonexistent() {
        let mut client = DiscuitClient::new("https://discuit.net");
        client.initialize().await.unwrap();

        let username = random::<u64>().to_string();
        let user = client.get_user_by_username(&username).await;

        match user {
            Ok(UserResponse::Error(APIError {
                status,
                code,
                message,
            })) => {
                assert_eq!(status, 404);
                assert_eq!(code, Some("user_not_found".to_string()));
                assert_eq!(message, "User not found.");
            }
            Ok(_) => panic!("Expected UserResponse::Error"),
            Err(e) => panic!("Expected Ok, got Err: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_get_feed() {
        let mut client = DiscuitClient::new("https://discuit.net");
        client.initialize().await.unwrap();

        let username =
            get_env_var("DISCUIT_USERNAME").expect("DISCUIT_USERNAME must be set for this test");
        let feed = client.get_feed(username.as_str()).await.unwrap();

        match feed {
            FeedResponse::Feed { feed, next } => {
                assert!(!feed.is_empty());
                assert!(next.is_some());
            }
            FeedResponse::Error(APIError {
                status,
                code: _,
                message,
            }) => {
                panic!("Failed to fetch feed: {} - {}", status, message);
            }
        }
    }

    #[tokio::test]
    async fn test_get_feed_nonexistent() {
        let mut client = DiscuitClient::new("https://discuit.net");
        client.initialize().await.unwrap();

        let username = random::<u64>().to_string();
        let feed = client.get_feed(&username).await;

        match feed {
            Ok(FeedResponse::Error(APIError {
                status,
                code,
                message,
            })) => {
                assert_eq!(status, 404);
                assert_eq!(code, Some("user_not_found".to_string()));
                assert_eq!(message, "User not found.");
            }
            Ok(_) => panic!("Expected FeedResponse::Error"),
            Err(e) => panic!("Expected Ok, got Err: {:?}", e),
        }
    }
}
