use super::connect::DB;
use eyre::Result;
use serde::{Deserialize, Serialize};

pub async fn insert_post(db: DB, text: &str, parent_id: Option<i32>) -> Result<i32> {
    let result = sqlx::query!(
        r#"
            INSERT INTO posts (text, parent_id) 
            SELECT $1, $2 
            RETURNING post_id;
        "#,
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
        r#"
            SELECT 
                p.post_id AS id, 
                p.text, 
                p.likes, 
                (
                    SELECT COUNT(*) FROM posts p2 WHERE p2.parent_id = p.post_id
                ) AS replies
            FROM posts p 
            WHERE p.parent_id is null 
            AND p.deleted_at IS NULL
            GROUP BY post_id;
        "#
    )
    .fetch_all(&db)
    .await?)
}

pub async fn get_one_post(db: DB, post_id: i32) -> Result<Option<ReplyPost>> {
    let db_post = sqlx::query_as!(
        DbPostWithReplies,
        r#"
                SELECT
                    post_id AS "id!",
                    text AS "text!",
                    likes AS "likes!",
                    parent_id,
                    (SELECT COUNT(*) FROM posts WHERE parent_id = $1) as count
                FROM posts
                WHERE post_id = $1
                AND deleted_at IS NULL
                GROUP BY "id!"
                UNION
                SELECT
                    p.post_id AS "id!",
                    p.text AS "text!",
                    p.likes AS "likes!",
                    p.parent_id,
                    (SELECT COUNT(*) FROM posts WHERE parent_id = p.post_id) as count
                FROM posts p
                WHERE parent_id = $1
                GROUP BY "id!"
                ORDER BY "id!" ASC;
        "#,
        post_id
    )
    .fetch_all(&db)
    .await?;

    let mut db_posts = db_post.into_iter();

    let Some(first) = db_posts.next() else {
        return Ok(None);
    };

    let id = first.id;

    if id != post_id {
        return Ok(None);
    }

    let text = first.text.clone();
    let likes = first.likes;
    let replies = db_posts
        .map(|db_post| Post {
            id: db_post.id,
            text: db_post.text,
            likes: db_post.likes,
            replies: db_post.count,
        })
        .collect();

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
    parent_id: Option<i32>,
    count: Option<i64>,
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

pub async fn update_post_text(db: DB, new_text: &str, post_id: i32) -> Result<()> {
    sqlx::query!(
        r#"
            UPDATE posts
            SET text = $1
            WHERE post_id = $2;
        "#,
        new_text,
        post_id
    )
    .execute(&db)
    .await?;

    Ok(())
}

pub async fn soft_delete_post(db: DB, post_id: i32) -> Result<()> {
    sqlx::query!(
        r#"
            UPDATE posts
            SET deleted_at = NOW()
            WHERE post_id = $1;
        "#,
        post_id
    )
    .execute(&db)
    .await?;

    Ok(())
}
