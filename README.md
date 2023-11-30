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

- cli (use `cargo install`)
  - sqlx-cli v0.7.3

## Setup

1. Create the dotenv file by copying the [.env_example](./.env_example) to .env. On a Unix-like system you can do this with the command `cp .env_example .env`
  1. Update the environment variables to whatever you want

## Database

A Docker compose file is included to spin up a Postgres database. If you have docker installed run the command `docker compose up -d` to start the database.

### Connecting to the database locally

We can connect to the database directly to check it by running the `psql` command in the docker container.

```sh
docker compose exec database psql -U postgres
```

### Models

#### Posts

| PK | FK | Name      | Type         |
|----|----|-----------|--------------|
| *  | *  | post_id   | serial       |
|    |    | text      | varchar(255) |
|    |    | parent_id | int          |
|    |    | likes     | int          |
