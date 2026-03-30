use regex::Regex;

#[allow(dead_code)]
pub fn extract_req(line: &str) -> Option<String> {
    let re = Regex::new(r"@req\s+(.+)").unwrap();
    re.captures(line)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().trim().to_string())
}
#[allow(dead_code)]
pub fn is_valid_req(line: &str) -> bool {
    let re = Regex::new(r"^REQ-\d+$").unwrap();
    re.is_match(line)
}
