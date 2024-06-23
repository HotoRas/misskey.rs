# Current Phases in details
This document will describe more details about this project, especially file side-by-side.

## Structure
- [x] index.ts > lib.rs
- [x] acct.ts > acct.rs
- [ ] api.ts > api.rs
- [ ] api.types.ts > api_types.rs
- [ ] consts.ts > consts.rs
- [ ] entities.ts > entities.rs
- [ ] index.ts > index.rs
- [ ] streaming.ts > streaming.rs
- [ ] streaming.types.ts > streaming_types.rs
- [ ] autogen
  - [ ] apiClientJSDoc.ts > apiClientJSDoc.rs
  - [ ] endpoint.ts > endpoint.rs
  - [ ] entities.ts > entities.rs
    - re-export from types.rs
    - [ ] EmptyRequest `Option<std::collections::HashMap<String, {Unknown}>>`
    - [ ] EmptyResponse `EmptyRequest`
  - [ ] models.ts > models.rs
    - re-export from types.rs
  - [ ] types.ts > types.rs
    - [x] Error
    - [ ] User
      - [x] UserLite
      - [ ] UserDetailedNotMeOnly
      - [ ] MeDetailedOnly
      - [x] UserDetailedNotMe
      - [x] MeDetailed
      - [x] UserDetailed
      - [x] User
    - [ ] UserList
    - [ ] Ad
    - [ ] Announcement
    - [ ] App
    - [ ] Note
    - [ ] NoteReaction
    - [ ] NoteFavorite
    - [ ] Notofication (The hardest part)
    - [ ] Drive
      - [ ] DriveFile
      - [ ] DriveFolder
    - [ ] Following and the Request
      - [ ] Following
      - [ ] FollowingRequest
    - [ ] Mute and Block
      - [ ] Muting
      - [ ] RenoteMuting
      - [ ] Blocking
    - [ ] #hashtag
    - [ ] InviteCode
    - [ ] Page
      - [ ] Page
      - [ ] PageBlock
    - [ ] Channel
    - [ ] Queue
    - [ ] Antenna
    - [ ] Clip
    - [ ] Federated Instance/Server
    - [ ] Gallery
    - [ ] Emoji
    - [ ] Flash
    - [ ] SignIn
    - [ ] Role and Conditions
    - [ ] Role Policies
    - [ ] Reversi
    - [ ] Metadata
    - [ ] Abuse User Report
    - [ ] Moderation Log
    - [ ] The Nevers (Rust have `Option<T>::Never`)