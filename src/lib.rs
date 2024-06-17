//! a library for easily interacting with the Discuit API.
//!
//! discuit-rs aims to provide 100% coverage of the Discuit API, making it easy to
//! interact with the Discuit API in Rust.
//!
//! # Quick Start
//!
//! ```rust
//! use std::env;
//! use discuit_rs::DiscuitClient;
//!
//! #[tokio::main]
//! async fn main() {
//!     let base_url = env::var("DISCUIT_BASE_URL").unwrap();
//!
//!     let mut client = DiscuitClient::new(&base_url);
//!     let response = client.initialize().await;
//!     println!("{:?}", response);
//! }
//! ```
//!
//! See the `examples` directory for more examples.
