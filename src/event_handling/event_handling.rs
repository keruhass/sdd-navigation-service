use std::path::Path;

use anyhow::Ok;

use crate::{analysis::data_analysis, models::Analysis};
pub async fn event_handling(
    mut rx: tokio::sync::mpsc::UnboundedReceiver<(notify::Event, String)>,
    analysis_tx: tokio::sync::mpsc::UnboundedSender<Analysis>,
) -> anyhow::Result<()> {
    while let Some((event, doc_path)) = rx.recv().await {
        for project_path in event.paths {
            println!("File was changed: {}", project_path.to_string_lossy());
            let analysis = data_analysis(&project_path.as_path(), Path::new(&doc_path)).await?;
            let _ = analysis_tx.send(analysis);
        }
    }

    Ok(())
}
