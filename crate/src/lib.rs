pub fn answer() -> usize {
    regex::Regex::new("a+").unwrap().find("baaa").map_or(0, |m| m.len())
}
