use axum::extract::State;
use axum::Json;

use crate::error::ApiResult;
use crate::models::TagRow;
use crate::state::AppState;

pub async fn list(State(state): State<AppState>) -> ApiResult<Json<Vec<TagRow>>> {
    let rows = sqlx::query_as::<_, TagRow>(
        "SELECT t.id, t.slug, t.name_en, t.name_cn, COUNT(pt.problem_id) AS problem_count
         FROM tags t
         LEFT JOIN problem_tags pt ON pt.tag_id = t.id
         GROUP BY t.id
         ORDER BY problem_count DESC, t.slug",
    )
    .fetch_all(&state.pool)
    .await?;
    Ok(Json(rows))
}
