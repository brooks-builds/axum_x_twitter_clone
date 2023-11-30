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

- Axum 0.7.1
