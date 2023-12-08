use crate::{database::queries::insert_post, state::AppState};
use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

pub async fn create_post(
    state: State<AppState>,
    post: CreatePost,
) -> Result<(StatusCode, Json<InsertedPost>), (StatusCode, &'static str)> {
    let post_id = insert_post(state.db.clone(), &post.text, post.parent_id)
        .await
        .map_err(|error| {
            tracing::error!("Error inserting post into database: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Post could not be created at this time, please try again later",
            )
        })?;

    Ok((StatusCode::CREATED, Json(InsertedPost { id: post_id })))
}

#[derive(Serialize)]
pub struct InsertedPost {
    pub id: i32,
}

#[derive(Debug)]
pub struct CreatePost {
    pub text: String,
    pub parent_id: Option<i32>,
}

#[async_trait]
impl FromRequest<AppState> for CreatePost
where
    Json<CreatePostPartial>: FromRequest<AppState, Rejection = JsonRejection>,
{
    type Rejection = Response;

    async fn from_request(request: Request, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(post) = Json::<CreatePostPartial>::from_request(request, state)
            .await
            .map_err(|error| {
                tracing::error!(
                    "Error extracting json body when creating post: {}",
                    error.body_text()
                );
                error.status().into_response()
            })?;

        let Some(text) = post.text else {
            return Err((StatusCode::BAD_REQUEST).into_response());
        };

        if text.is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                "Your post must have at least one character",
            )
                .into_response());
        }

        if text.len() > 255 {
            return Err((
                StatusCode::BAD_REQUEST,
                "Your post must be 255 characters or less",
            )
                .into_response());
        }

        if let Some(parent_id) = post.parent_id {
            if parent_id <= 0 {
                return Err(StatusCode::BAD_REQUEST.into_response());
            }
        }

        Ok(Self {
            text,
            parent_id: post.parent_id,
        })
    }
}

#[derive(Deserialize, Debug)]
pub struct CreatePostPartial {
    pub text: Option<String>,
    pub parent_id: Option<i32>,
}
