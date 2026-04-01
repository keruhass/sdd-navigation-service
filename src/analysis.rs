use std::path::Path;

use crate::models::Analysis;
use crate::models::ReqMap;
use crate::models::SpecMap;
use crate::scanner::scan_project;
use crate::specification_reader::scan_file;

//@req REQ-001
pub fn data_analysis(project_path: &Path, doc_path: &Path) -> anyhow::Result<Analysis> {
    let mut analysis = Analysis {
        covered: Vec::new(),
        uncovered: Vec::new(),
        unknown: Vec::new(),
    };
    let spec_map: SpecMap = scan_file(doc_path)?;
    let req_map: ReqMap = scan_project(project_path);

    for entry in spec_map.iter() {
        let key = entry.key();
        if req_map.contains_key(key) {
            analysis.covered.push(key.clone());
        } else {
            analysis.uncovered.push(key.clone());
        }
    }
    for entry in req_map.iter() {
        let key = entry.key();
        if !spec_map.contains_key(key) {
            analysis.unknown.push(key.clone());
        }
    }
    Ok(analysis)
}
