//! This module contains all API response structs.

use crate::structs::api_types::*;
use serde::{Deserialize, Serialize};

/// `APIError` represents an error returned by the Discuit API.
/// Most API errors return a JSON object of the following type, along with the appropriate HTTP status code:
/// ```json
/// {
///     "status": 400,
///     "code": "error_code",
///     "message": "Human readable error message"
/// }
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct APIError {
    /// The HTTP status code.
    pub status: i32,

    /// The error code.
    pub code: Option<String>,

    /// A human-readable error message.
    pub message: String,
}

/// `InitialResponse` represents the response from the `/api/_initial` endpoint.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct InitialResponse {
    /// Array of reasons for reporting a post or comment.
    #[serde(rename = "reportReasons")]
    pub report_reasons: Vec<ReportReason>,

    /// The user that is currently logged in. Null if the user is not logged in.
    pub user: Option<User>,

    /// Sequence of the user's lists.
    // TODO: Add List struct.
    pub lists: Option<Vec<String>>,

    /// Array of communities that the user is a member of.
    /// If the user is not logged in, the default communities are returned.
    pub communities: Vec<Community>,

    /// Total number of users on the platform.
    #[serde(rename = "noUsers")]
    pub no_users: i32,

    /// Array of communities that the user is banned from.
    /// Null if the user is not logged in.
    #[serde(rename = "bannedFrom")]
    pub banned_from: Option<Vec<Community>>,

    /// The public key for VAPID. This is used for push notifications.
    #[serde(rename = "vapidPublicKey")]
    pub vapid_public_key: String,

    /// Array of mutes that the user has.
    #[serde(rename = "mutes")]
    pub mutes: Mutes,
}

/// `UserResponse` represents the response from multiple endpoints that return a user object.
/// The body of the response is a JSON object that will be either of type
/// `User` or `APIError`.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum UserResponse {
    /// The user object.
    User(User),

    /// An API error.
    Error(APIError),
}

/// `PostFeedResponse` represents the response from the `/api/posts` endpoint.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct PostFeedResponse {
    /// Array of posts.
    pub posts: Vec<Post>,

    /// Pagination cursor. Null implies end of pagination.
    pub next: Option<String>,
}

/// `FeedResponse` represents the response from the `/api/users/{username}/feed` and
/// list endpoints. Returns a feed array + pagination cursor or an API error.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum FeedResponse {
    /// The feed array + pagination cursor.
    Feed {
        /// Array of feed items.
        feed: Vec<FeedItem>,

        /// Pagination cursor. Null implies end of pagination.
        next: Option<Next>,
    },

    /// An API error.
    Error(APIError),
}

/// `FeedItem` represents a post or comment in a feed.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum FeedItem {
    /// A post.
    Post(Post),

    /// A comment.
    Comment(Comment),
}

/// `Next` represents the pagination cursor in a feed response.
/// The cursor can be either a string or an integer, depending on the sort method.
/// If the sort method is `activity`, the cursor is an integer. Else, it is a string.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub enum Next {
    String(String),
    Int(i32),
}
