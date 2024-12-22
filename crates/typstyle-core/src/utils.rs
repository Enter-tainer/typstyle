use std::ops::Range;

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

/// Get the range of the string obtained from trimming in the original string.
pub fn trim_range(s: &str, mut rng: Range<usize>) -> Range<usize> {
    rng.end = rng.start + s[rng.clone()].trim_end().len();
    rng.start = rng.end - s[rng.clone()].trim_start().len();
    rng
}

pub fn count_spaces_after_last_newline(s: &str, i: usize) -> usize {
    // Ensure the byte position `i` is a valid UTF-8 boundary
    debug_assert!(
        s.is_char_boundary(i),
        "Position i is not a valid UTF-8 boundary"
    );

    // Find the last newline (`\n`) before position `i`
    if let Some(pos) = s[..i].rfind('\n') {
        // Get the substring after the newline and up to position `i`
        let after_newline = &s[pos + 1..i];
        // Count the number of consecutive spaces in the substring
        after_newline.chars().take_while(|&c| c == ' ').count()
    } else {
        // If no newline is found, return 0
        0
    }
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
