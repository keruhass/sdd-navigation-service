mod analysis;
mod handlers;
mod input;
mod models;
mod parser;
mod routes;
mod scanner;
mod specification_reader;
mod state;

use axum::Router;
use std::{path::Path, sync::Arc};

use crate::{analysis::data_analysis, input::path_input, routes::router};
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let doc_path = path_input();
    let project_path = path_input();

    let data = data_analysis(Path::new(&project_path), Path::new(&doc_path)).await?;

    let state = Arc::new(AppState { analysis: data });

    let app = Router::new().nest("/api", router()).with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await?;
    Ok(())
}
