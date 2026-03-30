use crate::models::{Location, ReqMap};
use crate::parser::{extract_req, is_valid_req};
use dashmap::DashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};
use walkdir::WalkDir;

pub fn scan_project<P: AsRef<Path>>(root: P) -> ReqMap {
    let mut map: ReqMap = DashMap::new();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();

        if !is_code_file(path) {
            continue;
        }

        if let Err(err) = scan_file(path, &mut map) {
            eprintln!("Failed to scan {:?}: {}", path, err);
        }
    }

    map
}

pub fn scan_file(path: &Path, map: &mut ReqMap) -> std::io::Result<()> {
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        let line: &str = &line?;
        let req = extract_req(line);

        if let Some(s) = path.to_str() {
            let path_str = s.to_string();

            let location = Location {
                file: path_str.clone(),
                line: i,
            };

            if let Some(req) = req {
                if is_valid_req(&req) {
                    map.insert(req, location);
                } else {
                    eprintln!(
                    "Annotation has wrong format. File: {}, line: {}.\nPlease, use the right format of the annotations: 'REQ-[number]'.",
                    &path_str, i
                    )
                }
            }
        }
    }

    Ok(())
}

fn is_code_file(path: &Path) -> bool {
    match path.extension().and_then(|s| s.to_str()) {
        Some(ext) => matches!(ext, "rs" | "ts" | "js" | "py"),
        None => false,
    }
}
