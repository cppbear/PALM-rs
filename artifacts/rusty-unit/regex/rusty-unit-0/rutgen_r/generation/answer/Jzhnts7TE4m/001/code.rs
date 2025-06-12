// Answer 0

#[derive(Debug)]
struct Pat {
    pattern: String,
}

impl Pat {
    fn new(pattern: &str) -> Self {
        Pat {
            pattern: pattern.to_string(),
        }
    }
}

struct Regex {
    pat: Pat,
}

impl Regex {
    fn new(pattern: &str) -> Self {
        Regex {
            pat: Pat::new(pattern),
        }
    }

    pub fn len(&self) -> usize {
        self.pat.pattern.len()
    }
}

#[test]
fn test_empty_regex() {
    let regex = Regex::new("");
    assert_eq!(regex.len(), 0);
}

#[test]
fn test_single_character_regex() {
    let regex = Regex::new("a");
    assert_eq!(regex.len(), 1);
}

#[test]
fn test_special_character_regex() {
    let regex = Regex::new(".*?");
    assert_eq!(regex.len(), 4);
}

#[test]
fn test_long_pattern_regex() {
    let regex = Regex::new("this is a longer regex pattern");
    assert_eq!(regex.len(), 31);
}

#[test]
fn test_whitespace_pattern_regex() {
    let regex = Regex::new("   ");
    assert_eq!(regex.len(), 3);
}

#[test]
fn test_non_empty_pattern_with_special_chars() {
    let regex = Regex::new("abc123!@#");
    assert_eq!(regex.len(), 10);
}

#[test]
fn test_multiple_special_chars() {
    let regex = Regex::new("(^[a-z]\\w*)?$");
    assert_eq!(regex.len(), 16);
}

