use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    database::queries::{self, ReplyPost},
    state::AppState,
};

pub async fn get_one_post(
    state: State<AppState>,
    Path(post_id): Path<i32>,
) -> Result<Json<ReplyPost>, (StatusCode, &'static str)> {
    let posts = queries::get_one_post(state.db.clone(), post_id)
        .await
        .map_err(|error| {
            tracing::error!("Error getting one post: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again",
            )
        })?;

    let Some(posts) = posts else {
        return Err((StatusCode::NOT_FOUND, "Post does not exist"));
    };

    Ok(Json(posts))
}
