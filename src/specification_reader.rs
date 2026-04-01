use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use dashmap::DashMap;

use crate::models::SpecMap;

pub fn scan_file(path: &Path) -> std::io::Result<SpecMap> {
    let map: SpecMap = DashMap::new();
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut pending_req: Option<(usize, String)> = None;

    for (i, line) in reader.lines().enumerate() {
        let line = line?;

        if line.starts_with("REQ") {
            if let Some((prev_i, prev_req)) = pending_req.take() {
                eprintln!(
                    "Line {} contains REQ without description: {}. Skip.",
                    prev_i, prev_req
                )
            }

            pending_req = Some((i, line))
        } else {
            match pending_req.take() {
                Some((_, req)) => {
                    map.insert(req, line);
                }
                None => {
                    eprintln!("Line {} contains description without REQ. Skip", i);
                }
            }
        }
    }

    if let Some((i, req)) = pending_req {
        eprintln!("Line {} doesn't have a description at EOF. REQ: {}", i, req);
    }

    Ok(map)
}
