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

/// `Mutes` represents the mutes that a user has, including community and user mutes.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct Mutes {
    /// Array of community mutes.
    #[serde(rename = "communityMutes")]
    pub community_mutes: Option<Vec<Mute>>,
    /// Array of user mutes.
    #[serde(rename = "userMutes")]
    pub user_mutes: Option<Vec<Mute>>,
}

/// `ReportReason` represents a reason for reporting a post or comment.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct ReportReason {
    /// The ID of the report reason.
    pub id: i32,
    /// The title of the report reason.
    pub title: String,
    /// The description of the report reason.
    pub description: Option<String>,
}

/// `User` represents a user on the platform.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
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
    #[serde(rename = "deleted")]
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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct Badge {
    /// The ID of the badge.
    pub id: i32,
    /// The title of the badge.
    #[serde(rename = "badgeTitle")]
    pub badge_type: String,
}

/// `Community` represents a community on the platform.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct ImageCopy {
    /// The name of the image used to identify it.
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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
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

/// `Comment` represents a comment on a post.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct Comment {
    /// The ID of the comment.
    pub id: String,
    /// The ID of the post the comment belongs to.
    #[serde(rename = "postId")]
    pub post_id: String,
    /// The public ID of the post the comment belongs to.
    #[serde(rename = "postPublicId")]
    pub post_public_id: String,

    /// The ID of the community in which this comment was made.
    #[serde(rename = "communityId")]
    pub community_id: String,
    /// The name of the community in which this comment was made.
    #[serde(rename = "communityName")]
    pub community_name: String,

    /// The ID of the user that made the comment.
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
    /// The username of the user that made the comment.
    pub username: String,
    /// The ID of the Ghost user in case the author deleted their account, otherwise undefined.
    #[serde(rename = "userGhostId")]
    pub user_ghost_id: Option<String>,
    /// The capacity in which the comment was created.
    #[serde(rename = "userGroup")]
    pub user_group: String,
    /// Indicates whether the author account is deleted.
    #[serde(rename = "userDeleted")]
    pub user_deleted: bool,

    /// The comment ID of the parent comment if it exists, otherwise null if this is a top-level comment.
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    /// How far deep into a comment chain this comment is. Top-level comments have a depth of 0.
    pub depth: i32,
    /// The total number of replies the comment has, including all deeper comments.
    #[serde(rename = "noReplies")]
    pub no_replies: i32,
    /// The number of direct replies the comment has. This does not include replies deeper than 1 more than the comment itself.
    #[serde(rename = "noDirectReplies")]
    pub no_direct_replies: i32,
    /// The comment IDs of all ancestor comments starting from the top-most comment.
    pub ancestors: Option<Vec<String>>,

    /// The body of the comment.
    pub body: String,
    /// The number of upvotes that the comment has.
    pub upvotes: i32,
    /// The number of downvotes that the comment has.
    pub downvotes: i32,
    /// The time at which the comment was created.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// If the comment was edited, the time at which it was last edited, otherwise null.
    #[serde(rename = "editedAt")]
    pub edited_at: Option<String>,

    /// If the content of the comment was deleted, otherwise undefined.
    #[serde(rename = "contentStripped")]
    pub content_stripped: Option<bool>,
    /// If the comment was deleted.
    pub deleted: bool,
    /// If the comment was deleted, the time at which it was deleted, otherwise null.
    #[serde(rename = "deletedAt")]
    pub deleted_at: Option<String>,
    /// If the comment was deleted, in what capacity it was deleted, otherwise undefined.
    #[serde(rename = "deletedAs")]
    pub deleted_as: Option<String>,

    /// The User object of the author of the comment.
    pub author: User,
    /// Whether the author is muted by the authenticated user. If not authenticated, this is undefined.
    #[serde(rename = "isAuthorMuted")]
    pub is_author_muted: Option<bool>,

    /// Indicated whether the authenticated user has voted. If not authenticated, this is null.
    #[serde(rename = "userVoted")]
    pub user_voted: Option<bool>,
    /// Indicates whether the authenticated user's vote is an upvote. If not authenticated, this is null.
    #[serde(rename = "userVotedUp")]
    pub user_voted_up: Option<bool>,

    /// The title of the post the comment belongs to.
    #[serde(rename = "postTitle")]
    pub post_title: Option<String>,
    /// Indicates whether the post the comment belongs to is deleted.
    #[serde(rename = "postDeleted")]
    pub post_deleted: bool,
    /// If the post is deleted, in what capacity, otherwise undefined.
    #[serde(rename = "postDeletedAs")]
    pub post_deleted_as: Option<String>,
}

/// `Post` represents a post.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct Post {
    /// The ID of the post.
    pub id: String,
    /// The type of post.
    #[serde(rename = "type")]
    pub post_type: String,
    /// The value in <https://discuit.net/gaming/post/{publicId}>
    #[serde(rename = "publicId")]
    pub public_id: String,

    /// ID of the author.
    #[serde(rename = "userId")]
    pub user_id: String,
    /// Username of the author.
    pub username: String,
    /// The ID of the Ghost user in case the user deleted their account.
    #[serde(rename = "userGhostId")]
    pub user_ghost_id: Option<String>,
    /// In what capacity the post was created. For "speaking officially" as a mod or an admin.
    #[serde(rename = "userGroup")]
    pub user_group: String,
    /// Indicated whether the author's account is deleted.
    #[serde(rename = "userDeleted")]
    pub user_deleted: bool,

    /// If the post is pinned in the community.
    #[serde(rename = "isPinned")]
    pub is_pinned: bool,
    /// If the post is pinned site-wide.
    #[serde(rename = "isPinnedSite")]
    pub is_pinned_site: bool,

    /// The ID of the community the post is posted in.
    #[serde(rename = "communityId")]
    pub community_id: String,
    /// The name of that community.
    #[serde(rename = "communityName")]
    pub community_name: String,
    /// The profile picture of that community.
    #[serde(rename = "communityProPic")]
    pub community_pro_pic: Image,
    /// The banner image of that community.
    #[serde(rename = "communityBannerImage")]
    pub community_banner_image: Image,

    /// Greater than 3 characters.
    pub title: String,
    /// Body of the post (only valid for text posts, null otherwise).
    pub body: Option<String>,
    /// The posted image (only valid for image posts, null otherwise).
    pub image: Option<Image>,
    /// The URL of the link.
    pub link: Option<Link>,

    /// If the post was locked.
    pub locked: bool,
    /// Who locked the post.
    #[serde(rename = "lockedBy")]
    pub locked_by: Option<String>,
    /// In what capacity the post was locked,
    #[serde(rename = "lockedByGroup")]
    pub locked_by_group: Option<String>,
    /// Time at which the post was locked.
    #[serde(rename = "lockedAt")]
    pub locked_at: Option<String>,

    /// The number of upvotes the post has.
    pub upvotes: i32,
    /// The number of downvotes the post has.
    pub downvotes: i32,
    /// For ordering posts by 'hot'.
    pub hotness: i32,

    /// The time when the post was created.
    pub created_at: String,
    /// Last edited time.
    #[serde(rename = "editedAt")]
    pub edited_at: Option<String>,
    /// Either the post created time or, if there are comments on the post, the time the most recent comment was created at.
    #[serde(rename = "lastActivityAt")]
    pub last_activity_at: String,

    /// If the post was deleted.
    pub deleted: bool,
    /// Time at which the post was deleted, null if the post has not been deleted.
    #[serde(rename = "deletedAt")]
    pub deleted_at: Option<String>,
    /// ID of the user who deleted the post.
    #[serde(rename = "deletedBy")]
    pub deleted_by: Option<String>,
    /// In what capacity the content was deleted, undefined if the content has not been deleted.
    #[serde(rename = "deletedAs")]
    pub deleted_as: Option<String>,
    /// If true, the body of the post and all associated links or images are deleted.
    #[serde(rename = "deletedContent")]
    pub deleted_content: bool,
    /// In what capacity the content was deleted, undefined if the content has not been deleted.
    #[serde(rename = "deletedContentAs")]
    pub deleted_content_as: Option<String>,

    /// Comment count.
    #[serde(rename = "noComments")]
    pub no_comments: i32,
    /// Comments of the post.
    pub comments: Option<Vec<Comment>>,
    /// Pagination cursor for comments.
    #[serde(rename = "commentsNext")]
    pub comments_next: Option<String>,

    /// Indicated whether the authenticated user has voted. If not authenticated, the value is null.
    #[serde(rename = "userVoted")]
    pub user_voted: Option<bool>,
    /// Indicated whether the authenticated user's vote is an upvote.
    #[serde(rename = "userUpvoted")]
    pub user_upvoted: Option<bool>,

    /// Indicated whether the author of the post has been muted by the logged in user.
    #[serde(rename = "isAuthorMuted")]
    pub is_author_muted: bool,
    /// Indicated whether the community the post is in has been muted by the logged in user.
    #[serde(rename = "isCommunityMuted")]
    pub is_community_muted: bool,

    /// The Community object of the community the post is in.
    pub community: Community,
    /// The User object of the author of the post.
    pub user: Option<User>,
}

/// `Link` represents a link
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct Link {
    /// The URL of the link.
    pub url: String,
    /// The hostname of the link. For a URL of "<https://discuit.net>", this would be "discuit.net".
    pub hostname: String,
    /// The image object of the OpenGraph image on the site. If no OpenGraph image was found, this is null.
    pub image: Option<Image>,
}

/// `List` represents a list.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct List {
    /// The ID of the list.
    pub id: i32,
    /// The ID of the list owner.
    #[serde(rename = "userId")]
    pub user_id: String,
    /// The username of the list owner.
    pub username: String,
    /// The name of the list.
    pub name: String,
    /// The display name of the list.
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// The description of the list. If no description was set, this is null.
    pub description: Option<String>,
    /// Indicates whether the list is a public or a private list.
    pub public: bool,
    /// Number of items in the list.
    #[serde(rename = "numItems")]
    pub num_items: i32,
    /// The current sorting of the list.
    pub sort: String,
    /// The time at which the list was created.
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    /// The last time an item was added to the list (for brand new lists this value is the same as createdAt).
    #[serde(rename = "lastUpdatedAt")]
    pub last_updated_at: DateTime<Utc>,
}

/// `ListItem` represents an item in a list.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct ListItem {
    /// The ID of the list item.
    pub id: i32,
    /// The ID of the list.
    #[serde(rename = "listId")]
    pub list_id: i32,
    /// The type of the target item.
    #[serde(rename = "targetType")]
    pub target_type: String,
    /// The ID of the target item.
    #[serde(rename = "targetId")]
    pub target_id: String,
    /// The time at which the list item was created.
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    /// The target item.
    #[serde(rename = "targetItem")]
    pub target_item: Post,
}

/// `Sort` represents the method used to sort posts.
pub enum Sort {
    Hot,
    Activity,
    New,
    Day,
    Week,
    Month,
    Year,
}

impl Sort {
    pub fn to_string(&self) -> String {
        match self {
            Sort::Hot => "hot".to_string(),
            Sort::Activity => "activity".to_string(),
            Sort::New => "new".to_string(),
            Sort::Day => "day".to_string(),
            Sort::Week => "week".to_string(),
            Sort::Month => "month".to_string(),
            Sort::Year => "year".to_string(),
        }
    }
}
