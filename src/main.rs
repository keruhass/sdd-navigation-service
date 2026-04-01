mod analysis;
mod handlers;
mod models;
mod parser;
mod routes;
mod scanner;
mod specification_reader;
mod state;

use axum::Router;
use std::{path::Path, sync::Arc};

use crate::{analysis::data_analysis, routes::router};
use state::AppState;

//@req REQ-001
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let doc_path: &Path = Path::new("src/test.md");
    let project_path: &Path = Path::new("./");

    let data = data_analysis(project_path, doc_path)?;

    let state = Arc::new(AppState { analysis: data });
    // println!("\n{:#?}", analysis);

    let app = Router::new().nest("/api", router()).with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await?;
    Ok(())
}
