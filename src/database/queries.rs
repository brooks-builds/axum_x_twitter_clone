use super::connect::DB;
use dotenvy::Error;
use eyre::Result;
use serde::{Deserialize, Serialize};

pub async fn insert_post(db: DB, text: &str, parent_id: Option<i32>) -> Result<i32> {
    let result = sqlx::query!(
        "INSERT INTO posts (text, parent_id) VALUES ($1, $2) RETURNING post_id;",
        text,
        parent_id
    )
    .fetch_one(&db)
    .await?;

    Ok(result.post_id)
}

pub async fn get_all_top_level(db: DB) -> Result<Vec<Post>> {
    Ok(sqlx::query_as!(
        Post,
        "SELECT p.post_id AS id, p.text, p.likes, COUNT((SELECT p2.post_id FROM posts p2 WHERE p2.parent_id = p.post_id)) AS replies FROM posts p WHERE p.parent_id is null GROUP BY post_id;"
    )
    .fetch_all(&db)
    .await?)
}

pub async fn get_one_post(db: DB, post_id: i32) -> Result<Option<ReplyPost>> {
    let db_post = sqlx::query!(
            "SELECT p.post_id AS id, p.text, p.likes, p2.text AS reply_text, p2.post_id AS reply_id, p2.likes AS reply_likes, COUNT((SELECT p3.post_id FROM posts p3 WHERE p3.parent_id = p2.post_id)) AS reply_replies FROM posts p LEFT JOIN posts p2 on p2.parent_id = p.post_id WHERE p.post_id = $1 GROUP BY p.post_id, p2.post_id;",
        post_id
        )
        .fetch_all(&db)
        .await?;

    let Some(first) = db_post.first() else {
        return Ok(None);
    };

    let id = first.id;
    let text = first.text.clone();
    let likes = first.likes;
    let replies = vec![];

    Ok(Some(ReplyPost {
        id,
        text,
        likes,
        replies,
    }))
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    id: i32,
    text: String,
    likes: i32,
    replies: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct DbPostWithReplies {
    id: i32,
    text: String,
    likes: i32,
    reply_id: Option<i32>,
    reply_text: Option<String>,
    reply_likes: Option<i32>,
    reply_replies: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct ReplyPost {
    id: i32,
    text: String,
    likes: i32,
    replies: Vec<Post>,
}

impl TryFrom<Vec<DbPostWithReplies>> for ReplyPost {
    type Error = eyre::Error;

    fn try_from(value: Vec<DbPostWithReplies>) -> std::result::Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(eyre::eyre!("db post with replies empty"));
        }

        let id = value[0].id;
        let text = value[0].text.clone();
        let likes = value[0].likes;
        let replies = vec![];

        Ok(Self {
            id,
            text,
            likes,
            replies,
        })
    }
}
