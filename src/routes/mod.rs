use std::sync::Arc;

use axum::Router;

use crate::state::AppState;

pub mod metrics;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().merge(metrics::router())
}
