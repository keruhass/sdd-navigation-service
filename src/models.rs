use dashmap::DashMap;

pub type ReqId = String;
pub type ReqDesc = String;

//@req REQ-002
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Location {
    pub file: String,
    pub line: usize,
}
#[derive(Debug, Clone)]
pub struct Analysis {
    pub covered: Vec<ReqId>,
    pub uncovered: Vec<ReqId>,
    pub unknown: Vec<ReqId>,
}
pub type ReqMap = DashMap<ReqId, Location>;
pub type SpecMap = DashMap<ReqId, ReqDesc>;
