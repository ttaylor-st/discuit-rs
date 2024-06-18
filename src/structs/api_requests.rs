//! This module contains all API request structs used in the `discuit-rs` library.

use serde::{Deserialize, Serialize};

/// `LoginRequest` is used by the /api/_login endpoint to log in to the
/// Discuit instance.
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
