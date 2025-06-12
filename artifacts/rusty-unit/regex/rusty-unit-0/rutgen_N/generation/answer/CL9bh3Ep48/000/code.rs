// Answer 0

#[cfg(test)]
struct Match;

struct Regex;

impl Regex {
    pub fn find(&self, _haystack: &[u8]) -> Option<Match> {
        None
    }
}

#[test]
fn test_find_returns_none() {
    let regex = Regex;
    let result = regex.find(b"some haystack");
    assert_eq!(result, None);
}

#[test]
fn test_find_empty_haystack() {
    let regex = Regex;
    let result = regex.find(b"");
    assert_eq!(result, None);
}

