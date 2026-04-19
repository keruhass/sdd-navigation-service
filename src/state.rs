use std::sync::Arc;

use tokio::sync::RwLock;

use crate::models::Analysis;

#[derive(Clone)]
pub struct AppState {
    pub analysis: Arc<RwLock<Analysis>>,
}
