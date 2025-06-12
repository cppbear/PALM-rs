// Answer 0

#[derive(Debug)]
struct Literals {
    items: Vec<String>,
}

#[derive(Debug)]
struct Matcher {
    literals: Vec<String>,
}

impl Matcher {
    fn prefixes(lits: &Literals) -> Self {
        Matcher {
            literals: lits.items.clone(),
        }
    }
}

struct Regex {
    literals: Literals,
    matcher: Matcher,
}

impl Regex {
    fn new(lits: Literals, matcher: Matcher) -> Self {
        Regex { literals: lits, matcher }
    }
}

#[test]
fn test_prefixes_empty_literals() {
    let lits = Literals { items: vec![] };
    let regex = Regex::prefixes(lits);
    assert_eq!(regex.matcher.literals.len(), 0);
}

#[test]
fn test_prefixes_single_literal() {
    let lits = Literals { items: vec!["test".to_string()] };
    let regex = Regex::prefixes(lits);
    assert_eq!(regex.matcher.literals.len(), 1);
    assert_eq!(regex.matcher.literals[0], "test");
}

#[test]
fn test_prefixes_multiple_literals() {
    let lits = Literals { items: vec!["foo".to_string(), "bar".to_string()] };
    let regex = Regex::prefixes(lits);
    assert_eq!(regex.matcher.literals.len(), 2);
    assert_eq!(regex.matcher.literals[0], "foo");
    assert_eq!(regex.matcher.literals[1], "bar");
}

