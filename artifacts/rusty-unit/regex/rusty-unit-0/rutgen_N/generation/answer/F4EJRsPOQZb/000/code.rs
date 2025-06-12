// Answer 0

#[derive(Debug)]
struct Literals {
    patterns: Vec<String>,
}

impl Literals {
    fn new(patterns: Vec<String>) -> Self {
        Self { patterns }
    }
}

#[derive(Debug)]
struct Matcher {
    suffixes: Vec<String>,
}

impl Matcher {
    fn suffixes(lits: &Literals) -> Self {
        let suffixes: Vec<String> = lits.patterns.iter()
            .map(|pattern| pattern.clone()) // In real use case, derive actual suffixes here
            .collect();
        Self { suffixes }
    }
}

struct Regex {
    literals: Literals,
    matcher: Matcher,
}

impl Regex {
    fn new(lits: Literals, matcher: Matcher) -> Self {
        Self { literals: lits, matcher }
    }

    pub fn suffixes(lits: Literals) -> Self {
        let matcher = Matcher::suffixes(&lits);
        Self::new(lits, matcher)
    }
}

#[test]
fn test_suffixes_empty() {
    let lits = Literals::new(vec![]);
    let regex = Regex::suffixes(lits);
    assert_eq!(regex.matcher.suffixes.len(), 0);
}

#[test]
fn test_suffixes_single() {
    let lits = Literals::new(vec!["test".into()]);
    let regex = Regex::suffixes(lits);
    assert_eq!(regex.matcher.suffixes.len(), 1);
    assert_eq!(regex.matcher.suffixes[0], "test");
}

#[test]
fn test_suffixes_multiple() {
    let lits = Literals::new(vec!["foo".into(), "bar".into(), "baz".into()]);
    let regex = Regex::suffixes(lits);
    assert_eq!(regex.matcher.suffixes.len(), 3);
    assert_eq!(regex.matcher.suffixes[0], "foo");
    assert_eq!(regex.matcher.suffixes[1], "bar");
    assert_eq!(regex.matcher.suffixes[2], "baz");
}

