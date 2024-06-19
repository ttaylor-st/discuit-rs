
# API Coverage

This does not include list related endpoints.


- [x] GET /_initial
- [ ] POST /push_subscriptions
- [ ] Authentication
  - [x] POST /_login
  - [ ] POST /_signup \* This can not be implemented, as it requires a
  reCAPTCHA token
- [ ] Communities
  - [ ] POST /_joinCommunityunity
  - [ ] GET /communities
  - [ ] GET /communities/{communityId}
  - [ ] PUT /communities/{communityId}
  - [ ] Mods
    - [ ] GET /communities/{communityId}/mods
    - [ ] POST /communities/{communityId}/mods
    - [ ] DELETE /communities/{communityId}/mods/{mod}
  - [ ] Rulse
    - [ ] GET /communities/{communityId}/rules
    - [ ] POST /communities/{communityId}/rules
    - [ ] DELETE /communities/{communityId}/rules/{ruleId}
    - [ ] PUT /communities/{communityId}/rules/{ruleId}
- [ ] Notifications
  - [ ] GET /notifications
  - [ ] POST /notifications
    -  [ ] ?action=resetNewCount
    -  [ ] ?action=markAllAsSeen
    -  [ ] ?action=deleteAll
  - [ ] GET /notifications/{notificationId}
  - [ ] PUT /notifications/{notificationId}
  - [ ] DELETE /notifications/{notificationId}
- [ ] Posts
  - [ ] POST /_postVote
  - [ ] GET /posts
  - [ ] POST /posts
  - [ ] GET /posts/{postId}
  - [ ] PUT /posts/{postId}
  - [ ] DELETE /posts/{postId}
  - [ ] Comments
    - [ ] POST /_commentVote
    - [ ] GET /posts/{postId}/comments
    - [ ] POST /posts/{postId}/comments
    - [ ] PUT /posts/{postId}/comments/{commentId}
    - [ ] DELETE /posts/{postId}/comments/{commentId}
- [ ] Users
  - [ ] POST /_settings
  - [x] GET /_user
  - [x] GET /users/{username}
  - [ ] GET /users/{username}/feed
