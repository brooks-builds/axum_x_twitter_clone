use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use crate::{database::queries, state::AppState};

pub async fn delete_post(
    Path(post_id): Path<i32>,
    state: State<AppState>,
) -> Result<StatusCode, (StatusCode, &'static str)> {
    queries::soft_delete_post(state.db.clone(), post_id)
        .await
        .map_err(|error| {
            tracing::error!("Error soft deleting post: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error deleting the post, please try again later",
            )
        })?;

    Ok(StatusCode::NO_CONTENT)
}
