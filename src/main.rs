mod analysis;
mod handlers;
mod models;
mod parser;
mod scanner;
mod specification_reader;

use std::path::Path;

use crate::analysis::data_analysis;
use axum::Router;

//@req REQ-001
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let doc_path: &Path = Path::new("src/test.md");
    let project_path: &Path = Path::new("./");

    let analysis = data_analysis(project_path, doc_path);

    println!("\n{:#?}", analysis);

    Ok(())
}
