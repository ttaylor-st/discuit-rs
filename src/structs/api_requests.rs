//! This module contains all API request structs used in the `discuit-rs` library.

use serde::{Deserialize, Serialize};

/// `LoginRequest` is used by the /api/_login endpoint to log in to the
/// Discuit instance.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
