use std::path::Path;

use notify::event::ModifyKind;
use notify::{recommended_watcher, EventKind, RecursiveMode, Watcher};
use tokio::sync::mpsc;

fn is_ignored(path: &Path) -> bool {
    path.components().any(|c| c.as_os_str() == "target")
}

pub fn event_watcher(
    project_path: String,
    doc_path: String,
    tx: mpsc::UnboundedSender<(notify::Event, String)>,
) {
    std::thread::spawn(move || -> notify::Result<()> {
        let (notify_tx, notify_rx) = std::sync::mpsc::channel();

        let mut watcher = recommended_watcher(notify_tx)?;
        watcher.watch(Path::new(&project_path), RecursiveMode::Recursive)?;

        for res in notify_rx {
            if let Ok(event) = res {
                let filtered_paths: Vec<_> =
                    event.paths.into_iter().filter(|p| !is_ignored(p)).collect();

                if filtered_paths.is_empty() {
                    continue;
                }

                let event = notify::Event {
                    paths: filtered_paths,
                    ..event
                };
                match event.kind {
                    EventKind::Modify(ModifyKind::Data(_)) => {
                        // реально изменилось содержимое файла
                        let _ = tx.send((event, doc_path.clone()));
                    }
                    _ => {
                        // игнорируем весь шум
                    }
                }
            }
        }

        Ok(())
    });
}
