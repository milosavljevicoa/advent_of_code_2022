pub fn convert(line: &str) -> Vec<&str> {
    line.split("\n").filter(|s| !s.is_empty()).collect()
}
