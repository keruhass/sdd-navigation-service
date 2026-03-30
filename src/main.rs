mod models;
mod parser;
mod scanner;
mod specification_reader;

use std::path::Path;

use dashmap::DashMap;

use crate::models::SpecMap;
use crate::scanner::scan_project;
use crate::specification_reader::scan_file;

//@req REQ-001
fn main() -> std::io::Result<()> {
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

    Ok(())
}
