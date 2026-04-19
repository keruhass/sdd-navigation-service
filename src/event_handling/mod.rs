use crate::{
    event_handling::{change_watcher::event_watcher, event_handling::event_handling},
    models::Analysis,
};

pub mod change_watcher;
pub mod event_handling;

pub async fn start_event_watcher(
    project_path: String,
    doc_path: String,
) -> tokio::sync::mpsc::UnboundedReceiver<Analysis> {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    let (analysis_tx, analysis_rx) = tokio::sync::mpsc::unbounded_channel();

    event_watcher(project_path, doc_path, tx);
    tokio::spawn(async move {
        let _ = event_handling(rx, analysis_tx).await;
    });

    analysis_rx
}
