use crate::models::{Location, ReqMap};
use crate::parser::{extract_req, is_valid_req};
use dashmap::DashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::task::JoinHandle;
use walkdir::WalkDir;

pub async fn scan_project<P: AsRef<Path>>(root: P) -> anyhow::Result<Arc<ReqMap>> {
    let map: Arc<ReqMap> = Arc::new(DashMap::new());
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.into_path();
        let map = map.clone();
        if !is_code_file(&path) {
            continue;
        }

        let handle = tokio::spawn(async move {
            if let Err(err) = scan_file(path, map).await {
                eprintln!("An error has occured while reading the file: {}", err);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await?;
    }

    Ok(map)
}

async fn scan_file(path: PathBuf, map: Arc<ReqMap>) -> std::io::Result<()> {
    let file = File::open(&path).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut line_index = 0;

    while let Some(line) = lines.next_line().await? {
        let line = line.as_str();
        let req = extract_req(line);

        if let Some(s) = path.to_str() {
            let path_str = s.to_string();

            let location = Location {
                file: path_str.clone(),
                line: line_index,
            };

            if let Some(req) = req {
                if is_valid_req(&req) {
                    map.insert(req, location);
                } else {
                    eprintln!(
                    "Annotation has wrong format. File: {}, line: {}.\nPlease, use the right format of the annotations: 'REQ-[number]'.",
                    &path_str, line_index
                    )
                }
            }
        }

        line_index += 1;
    }

    Ok(())
}

fn is_code_file(path: &PathBuf) -> bool {
    match path.extension().and_then(|s| s.to_str()) {
        Some(ext) => matches!(ext, "rs" | "ts" | "js" | "py"),
        None => false,
    }
}
