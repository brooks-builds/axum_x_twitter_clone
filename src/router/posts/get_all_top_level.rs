use axum::{extract::State, http::StatusCode, Json};

use crate::database::queries;
use crate::{database::queries::Post, state::AppState};

pub async fn get_all_top_level(
    state: State<AppState>,
) -> Result<Json<Vec<Post>>, (StatusCode, &'static str)> {
    let posts = queries::get_all_top_level(state.db.clone())
        .await
        .map_err(|error| {
            tracing::error!("Error getting all top level posts: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was a problem getting all posts, please try again later",
            )
        })?;

    Ok(Json(posts))
}
