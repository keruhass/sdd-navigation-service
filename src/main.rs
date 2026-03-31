mod models;
mod parser;
mod scanner;
mod specification_reader;

use std::path::Path;

use dashmap::DashMap;

use crate::models::{Analysis, SpecMap};
use crate::scanner::scan_project;
use crate::specification_reader::scan_file;

//@req REQ-001
fn main() -> std::io::Result<()> {
    let mut analysis = Analysis {
        covered: Vec::new(),
        uncovered: Vec::new(),
        unknown: Vec::new(),
    };

    let path: &Path = Path::new("src/test.md");
    let mut map: SpecMap = DashMap::new();

    scan_file(path, &mut map)?;

    for entry in map.iter() {
        println!("{}, {}", entry.key(), entry.value());
    }

    println!("Reqs in docs:");

    let doc_map = scan_project("./");

    for entry in doc_map.iter() {
        println!("{}, \n{:#?}", entry.key(), entry.value());
    }

    for entry in map.iter() {
        let key = entry.key();
        if doc_map.contains_key(key) {
            analysis.covered.push(key.clone());
        } else {
            analysis.uncovered.push(key.clone());
        }
    }
    for entry in doc_map.iter() {
        let key = entry.key();
        if !map.contains_key(key) {
            analysis.unknown.push(key.clone());
        }
    }

    println!("\n{:#?}", analysis);

    Ok(())
}
