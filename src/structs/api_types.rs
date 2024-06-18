//! This module contains the types that are used and returned by the Discuit API.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// TODO: Add missing fields to the structs.
// - [ ] Comment
// - [X] Community
// - [X] Image
//   - [x] ImageCopy
// - [ ] Mute
// - [ ] Notification
// - [ ] Post
// - [ ] List
// - [ ] Report
// - [X] ReportReason
// - [X] User
// - [X] Response (InitResponse)

/// `InitialResponse` represents the response from the `/api/_initial` endpoint.
#[derive(Debug, Serialize, Deserialize)]
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

/// `Mutes` represents the mutes that a user has, including community and user mutes.
#[derive(Serialize, Deserialize, Debug)]
pub struct Mutes {
    /// Array of community mutes.
    #[serde(rename = "communityMutes")]
    pub community_mutes: Option<Vec<Mute>>,
    /// Array of user mutes.
    #[serde(rename = "userMutes")]
    pub user_mutes: Option<Vec<Mute>>,
}

/// `ReportReason` represents a reason for reporting a post or comment.
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportReason {
    /// The ID of the report reason.
    pub id: i32,
    /// The title of the report reason.
    pub title: String,
    /// The description of the report reason.
    pub description: Option<String>,
}

/// `User` represents a user on the platform.
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// The ID of the user.
    pub id: String,
    /// The username of the user. Minimum 3 characters. Maximum 21 characters.
    pub username: String,
    /// If an email address was provided, the email address of the user, otherwise null.
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// If the email address was confirmed, the time at which it was confirmed, otherwise null.
    #[serde(rename = "emailConfirmedAt")]
    pub email_confirmed_at: Option<DateTime<Utc>>,
    /// The about set by the user. Maximum 10000 characters. If no about was set, this is null.
    #[serde(rename = "aboutMe")]
    pub about_me: Option<String>,
    /// The number of points that the user has.
    pub points: i32,
    /// If the user is an admin.
    #[serde(rename = "isAdmin")]
    pub is_admin: bool,
    /// If a profile picture was set, the profile picture of the user, otherwise null.
    #[serde(rename = "proPic")]
    pub pro_pic: Option<Image>,
    /// The list of badges that the user has, can be empty.
    #[serde(rename = "badges")]
    pub badges: Vec<Badge>,
    /// The number of posts the user has made.
    #[serde(rename = "noPosts")]
    pub no_posts: i32,
    /// The number of comments the user has made.
    #[serde(rename = "noComments")]
    pub no_comments: i32,
    /// The time at which the account was created.
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    /// If the account has been deleted.
    #[serde(rename = "isDeleted")]
    pub deleted: bool,
    /// If the account was deleted, the time at which it was deleted, otherwise null.
    #[serde(rename = "deletedAt")]
    pub deleted_at: Option<DateTime<Utc>>,
    /// If the user has turned off upvote notifications.
    #[serde(rename = "upvoteNotificationsOff")]
    pub upvote_notifications_off: bool,
    /// If the user has turned off reply notifications.
    #[serde(rename = "replyNotificationsOff")]
    pub reply_notifications_off: bool,
    /// The feed the user has set as their home feed.
    #[serde(rename = "homeFeed")]
    pub home_feed: String,
    /// If the user wants their feed sort to be remembered.
    #[serde(rename = "rememberFeedSort")]
    pub remember_feed_sort: bool,
    /// If the user wants to turn off embeds for link posts.
    #[serde(rename = "embedsOff")]
    pub embeds_off: bool,
    /// If the user wants to hide other users' profile pictures.
    #[serde(rename = "hideUserProfilePictures")]
    pub hide_user_profile_pictures: bool,
    /// If the user was banned, the time at which they were banned, otherwise null.
    #[serde(rename = "bannedAt")]
    pub banned_at: Option<DateTime<Utc>>,
    /// If the user was banned.
    #[serde(rename = "isBanned")]
    pub is_banned: bool,
    /// The number of new notifications the user has.
    #[serde(rename = "notificationsNewCount")]
    pub notifications_new_count: i32,
    /// If the user is a moderator in any communities, the list of communities that the user moderates, otherwise null.
    #[serde(rename = "moddingList")]
    pub modding_list: Option<Vec<Community>>,
}

/// `Badge` represents a badge that a user has.
#[derive(Debug, Serialize, Deserialize)]
pub struct Badge {
    /// The ID of the badge.
    pub id: i32,
    /// The title of the badge.
    #[serde(rename = "badgeTitle")]
    pub badge_type: String,
}

/// `Community` represents a community on the platform.
#[derive(Debug, Serialize, Deserialize)]
pub struct Community {
    /// The ID of the community.
    pub id: String,
    /// ID of the user who created the community.
    #[serde(rename = "userId")]
    pub user_id: String,

    /// The name of the community.
    pub name: String,
    /// If the community hosts NSFW content.
    pub nsfw: bool,
    /// The description of the community, null if no description was set. Maximum 2000 characters.
    pub about: Option<String>,
    /// The number of members of the community.
    #[serde(rename = "noMembers")]
    pub no_members: i32,

    /// The community icon.
    #[serde(rename = "proPic")]
    pub pro_pic: Option<Image>,
    /// The community banner image.
    #[serde(rename = "bannerImage")]
    pub banner_image: Option<Image>,

    /// The time at which the community was created.
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    /// If the community was deleted, the time at which it was deleted, otherwise null.
    #[serde(rename = "deletedAt")]
    pub deleted_at: Option<DateTime<Utc>>,

    /// If the community is a default community, only returned if the default communities are requested.
    #[serde(rename = "isDefault")]
    pub is_default: Option<bool>,

    /// Indicates whether the authenticated user is a member. If not authenticated, this is null.
    #[serde(rename = "userJoined")]
    pub user_joined: Option<bool>,
    /// Indicates whether the authenticated user is a moderator. If not authenticated, this is null.
    #[serde(rename = "userMod")]
    pub user_mod: Option<bool>,

    /// The User objects of all of the moderators of the community.
    pub mods: Option<Vec<User>>,
    /// The list of community rules. The list is empty if there are no rules.
    pub rules: Option<Vec<CommunityRule>>,

    /// Only visible to moderators of the community, otherwise null.
    #[serde(rename = "reportsDetails")]
    pub report_details: Option<ReportDetails>,
}

/// `ReportDetails` represents the details of reports on a community.
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportDetails {
    /// The total number of reports.
    #[serde(rename = "noReports")]
    pub no_reports: i32,
    /// The total number of posts reported.
    #[serde(rename = "noPostReports")]
    pub no_post_reports: i32,
    /// The total number of comments reported.
    #[serde(rename = "noCommentReports")]
    pub no_comment_reports: i32,
}

/// `CommunityRule` represents a rule of a community.
#[derive(Debug, Serialize, Deserialize)]
pub struct CommunityRule {
    /// The ID of the community rule.
    pub id: i32,

    /// The title of the rule.
    pub rule: String,
    /// The description of the rule. If no description was set, this is null.
    pub description: Option<String>,

    /// The ID of the community in which this is a rule.
    #[serde(rename = "communityId")]
    pub community_id: String,
    /// The index of the rule. A smaller value means that the rule is closer to the top.
    #[serde(rename = "zIndex")]
    pub z_index: i32,

    /// The ID of the user that created the rule.
    #[serde(rename = "createdBy")]
    pub created_by: String,
    /// The time at which the rule was created.
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

/// `Image` represents an image.
#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    /// The ID of the image.
    pub id: String,

    /// The image format.
    pub format: String,
    /// The image MIME Type, eg. "image/jpeg".
    pub mimetype: String,

    /// The image width.
    pub width: i32,
    /// The image height.
    pub height: i32,
    /// The size of the image in bytes.
    pub size: i32,

    /// The average color of the image.
    #[serde(rename = "averageColor")]
    pub average_color: String,

    /// A link to the image. The path is not prefixed with /api.
    pub url: String,
    /// A list of copies of the image in different sizes.
    pub copies: Vec<ImageCopy>,
}

/// `ImageCopy` represents a copy of an `Image` in a different size.
#[derive(Debug, Serialize, Deserialize)]
pub struct ImageCopy {
    /// The name of the image copy, used to identify it.
    pub name: Option<String>,

    /// The width of the image copy.
    pub width: i32,
    /// The height of the image copy.
    pub height: i32,
    /// The width of the box that the image fits into.
    #[serde(rename = "boxWidth")]
    pub box_width: i32,
    /// The height of the box that the image fits into.
    #[serde(rename = "boxHeight")]
    pub box_height: i32,
    /// How the image should fit into a box. Corresponds to the CSS `object-fit` property.
    #[serde(rename = "objectFit")]
    pub object_fit: String,

    /// The format of the image copy.
    pub format: String,
    /// A link to the image copy. The path is not prefixed with /api.
    pub url: String,
}

/// `Mute` represents a mute.
#[derive(Debug, Serialize, Deserialize)]
pub struct Mute {
    /// The ID of the mute.
    pub id: String,
    /// Whether a user or community is being muted.
    #[serde(rename = "type")]
    pub mute_type: String,
    /// If a user is being muted, the ID of the user, otherwise undefined.
    #[serde(rename = "mutedUserId")]
    pub muted_user_id: Option<String>,
    /// If a community is being muted, the ID of the community, otherwise undefined.
    #[serde(rename = "mutedCommunityId")]
    pub muted_community_id: Option<String>,

    /// The time at which the mute was created.
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,

    /// If a user is being muted, the User object of the user, otherwise undefined.
    #[serde(rename = "mutedUser")]
    pub muted_user: Option<User>,
    /// If a community is being muted, the Community object of the community, otherwise undefined.
    #[serde(rename = "mutedCommunity")]
    pub muted_community: Option<Community>,
}
