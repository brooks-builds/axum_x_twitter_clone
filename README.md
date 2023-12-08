# X / Twitter Clone

If we removed everything other than the basics of x/twitter, we have a micro-blogging platform with the following features which we'll be implementing.

## Features

- [x] Create a post
  - [x] limit the characters per post
  - [x] optionally can be response to another post
  - [x] optionally can be response to response
  - [x] posts are stored in postgres
  - [x] post are validated before storing in database
- [x] get a list of all top-level posts
  - [x] text
  - [x] likes
  - [x] count of immediate children
  - [x] deleted posts are ignored
- [x] get one post
  - [x] get immediate responses to the post
  - [x] text
  - [x] likes
  - [x] deleted posts respond with a 404
- [x] update post
  - [x] text
- [x] delete post
  - [x] soft delete post

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
- serde v1.0.193
  - with features
    - derive
- sqlx v0.7.3
  - with features
    - postgres
    - runtime-tokio-rustls

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

| PK | FK | Name      | Type         | Nullable | Default |
|----|----|-----------|--------------| | |
| *  | *  | post_id   | serial       | | |
|    |    | text      | varchar(255) | | |
|    |    | parent_id | int          | * | |
|    |    | likes     | int          | | 0 |

## Testing

The following are CURL commands to test the API. You can run them in the terminal __if__ you have `curl` installed. Otherwise you might be able to import them into your favorite API testing tool like Postman.

```sh
## create post
curl -X "POST" "http://localhost:33498/api/v1/posts" \
     -H 'Content-Type: application/json; charset=utf-8' \
     -d $'{
  "text": "I am a new post",
}'
## reply to a post
curl -X "POST" "http://localhost:33498/api/v1/posts" \
     -H 'Content-Type: application/json; charset=utf-8' \
     -d $'{
  "text": "I am a reply post",
  "parent_id": 1
}'

## get all top level
curl "http://localhost:33498/api/v1/posts"

## delete post
curl -X "DELETE" "http://localhost:33498/api/v1/posts/1"

## update
curl -X "PATCH" "http://localhost:33498/api/v1/posts/4" \
     -H 'Content-Type: application/json; charset=utf-8' \
     -d $'{
  "text": "I have been renamed"
}'

## get one post
curl "http://localhost:33498/api/v1/posts/15"

```
