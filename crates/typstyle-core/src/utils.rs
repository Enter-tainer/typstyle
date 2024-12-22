/// Strip trailing whitespace in each line of the input string.
pub fn strip_trailing_whitespace(s: &str) -> String {
    if s.is_empty() {
        return "\n".to_string();
    }
    let mut res = String::with_capacity(s.len());
    for line in s.lines() {
        res.push_str(line.trim_end());
        res.push('\n');
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_trailing_whitespace() {
        let s = strip_trailing_whitespace("");
        assert_eq!(s, "\n");
        let s = strip_trailing_whitespace(" ");
        assert_eq!(s, "\n");
        let s = strip_trailing_whitespace("\n");
        assert_eq!(s, "\n");
        let s = strip_trailing_whitespace(" \n - \n");
        assert_eq!(s, "\n -\n");
        let s = strip_trailing_whitespace(" \n - \n ");
        assert_eq!(s, "\n -\n\n");
    }
}
