mod analysis;
mod event_handling;
mod handlers;
mod input;
mod models;
mod parser;
mod routes;
mod scanner;
mod specification_reader;
mod state;

use std::{path::Path, sync::Arc};

use axum::Router;
use tokio::sync::{mpsc, RwLock};

use crate::{
    analysis::data_analysis, event_handling::start_event_watcher, input::path_input, routes::router,
};
use state::AppState;

//@req REQ-005
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let doc_path = path_input();
    let project_path = path_input();

    let data = data_analysis(Path::new(&project_path), Path::new(&doc_path)).await?;

    let state = Arc::new(RwLock::new(data));

    let mut rx = start_event_watcher(project_path.clone(), doc_path.clone()).await;

    let state_clone = state.clone();

    tokio::spawn(async move {
        while let Some(analysis) = rx.recv().await {
            let mut state = state_clone.write().await;
            *state = analysis;
        }
    });

    let app_state = Arc::new(AppState { analysis: state });

    let app = Router::new().nest("/api", router()).with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await?;
    Ok(())
}
