use axum::{
    async_trait,
    body::Bytes,
    extract::{rejection::JsonRejection, FromRequest, Request},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Error, Json,
};
use serde::Deserialize;

pub async fn create_post(post: CreatePost) {
    tracing::debug!("{post:?}");
}

#[derive(Debug)]
pub struct CreatePost {
    pub text: String,
}

#[async_trait]
impl<S> FromRequest<S> for CreatePost
where
    Json<CreatePostPartial>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
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

        Ok(Self { text })
    }
}

#[derive(Deserialize, Debug)]
pub struct CreatePostPartial {
    pub text: Option<String>,
}
