use dashmap::DashMap;

pub type ReqId = String;
pub type ReqDesc = String;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Location {
    pub file: String,
    pub line: usize,
}
#[allow(dead_code)]
pub struct Analysis {
    pub covered: Vec<ReqId>,
    pub uncovered: Vec<ReqId>,
    pub unknown: Vec<ReqId>,
}
#[allow(dead_code)]
pub type ReqMap = DashMap<ReqId, Location>;
#[allow(dead_code)]
pub type SpecMap = DashMap<ReqId, ReqDesc>;
