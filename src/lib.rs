//! a library for easily interacting with the Discuit API.
//!
//! discuit-rs aims to provide 100% coverage of the Discuit API, making it easy to
//! interact with the Discuit API in Rust.
//!
//! # Quick Start
//!
//! This example was taken from `examples/quickstart/main.rs`.
//!
//! ```rust
//! use std::env;
//! use discuit_rs::client::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let base_url = "https://discuit.net";
//!
//!     let mut client = DiscuitClient::new(&base_url);
//!     let response = client.initialize().await;
//!     println!("{:?}", response);
//! }
//! ```
//!
//! You can find more examples in the `examples` directory. To run the examples, use the following command:
//! ```sh
//! cargo run --example <example_name>
//! ```

/// The structs module contains all structs and enums used in the `discuit-rs` library.
pub mod structs {
    /// The `api_types` module contains all types used in the `discuit-rs` library.
    pub mod api_types;

    /// The `api_responses` module contains all API response structs used in the `discuit-rs` library.
    pub mod api_responses;

    /// The `api_requests` module contains all API request structs used in the `discuit-rs` library.
    pub mod api_requests;

    /// The `internal_types` module contains all internal types used in the `discuit-rs` library.
    pub mod internal_types;
}

/// The client module contains the `DiscuitClient` struct, which is used to interact with the Discuit API.
pub mod client;
