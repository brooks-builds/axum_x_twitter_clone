use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{database::queries, state::AppState};

pub async fn update_post_text(
    Path(post_id): Path<i32>,
    state: State<AppState>,
    Json(update_post_text): Json<UpdatePostText>,
) -> Result<StatusCode, (StatusCode, &'static str)> {
    queries::update_post_text(state.db.clone(), &update_post_text.text, post_id)
        .await
        .map_err(|error| {
            tracing::error!("Error updating post text: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error updating the post text, please try again",
            )
        })?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Serialize, Deserialize)]
pub struct UpdatePostText {
    pub text: String,
}
