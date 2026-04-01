use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{models::Analysis, state::AppState};

//@req REQ-002
pub async fn get_analysis(State(state): State<Arc<AppState>>) -> Json<Analysis> {
    let response = state.analysis.clone();
    Json(response)
}
