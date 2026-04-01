use std::sync::Arc;

use axum::routing::get;
use axum::Router;

use crate::handlers::metrics::get_analysis;
use crate::state::AppState;

//@req REQ-003
pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/metrics/analysis", get(get_analysis))
}
