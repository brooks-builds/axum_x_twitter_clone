# X / Twitter Clone

If we removed everything other than the basics of x/twitter, we have a micro-blogging platform with the following features which we'll be implementing.

## Features

- [ ] Create a post
  - [ ] limit the characters per post
  - [ ] optionally can be response to another post
  - [ ] optionally can be response to response
  - [ ] posts are stored in postgres
  - [ ] post are validated before storing in database
- [ ] get a list of all top-level posts
  - [ ] text
  - [ ] likes
  - [ ] paginate
- [ ] get one post
  - [ ] get immediate responses to the post
    - [ ] paginate
  - [ ] text
  - [ ] likes
- [ ] update post
  - [ ] text
  - [ ] undelete
- [ ] delete post
  - [ ] soft delete post

## Tech

- Axum v0.7.1
- dotenvy v0.15.7
- eyre v0.6.9
- tokio v1.34.0
  - with features
    - net
    - rt-multi-thread
    - macros
- tracing v0.1.40
- tracing-subscriber v0.3.18
- tower-http v0.5.0
  - with features
    - trace
