// Answer 0

#[derive(Debug)]
struct Literal {
    value: String,
}

impl Literal {
    fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

struct LiteralSet {
    lits: Vec<Literal>,
}

impl LiteralSet {
    fn new(lits: Vec<Literal>) -> Self {
        LiteralSet { lits }
    }

    fn contains_empty(&self) -> bool {
        self.lits.iter().any(|lit| lit.is_empty())
    }
}

#[test]
fn test_contains_empty_with_empty_literal() {
    let set = LiteralSet::new(vec![Literal { value: String::from("") }]);
    assert!(set.contains_empty());
}

#[test]
fn test_contains_empty_without_empty_literal() {
    let set = LiteralSet::new(vec![Literal { value: String::from("a") }]);
    assert!(!set.contains_empty());
}

#[test]
fn test_contains_empty_with_mixed_literals() {
    let set = LiteralSet::new(vec![
        Literal { value: String::from("a") },
        Literal { value: String::from("") },
        Literal { value: String::from("b") },
    ]);
    assert!(set.contains_empty());
}

#[test]
fn test_contains_empty_with_multiple_empty_literals() {
    let set = LiteralSet::new(vec![
        Literal { value: String::from("") },
        Literal { value: String::from("") },
    ]);
    assert!(set.contains_empty());
}

