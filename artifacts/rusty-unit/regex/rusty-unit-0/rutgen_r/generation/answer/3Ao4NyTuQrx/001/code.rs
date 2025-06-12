// Answer 0


struct Literal {
    content: String,
}

impl Literal {
    fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

struct LiteralSet {
    lits: Vec<Literal>,
}

impl LiteralSet {
    pub fn new(lits: Vec<Literal>) -> Self {
        LiteralSet { lits }
    }

    pub fn contains_empty(&self) -> bool {
        self.lits.iter().any(|lit| lit.is_empty())
    }
}

#[test]
fn test_contains_empty_with_empty_literal() {
    let empty_literal = Literal { content: String::new() }; // Empty literal
    let set_with_empty = LiteralSet::new(vec![empty_literal]);
    assert!(set_with_empty.contains_empty());
}

#[test]
fn test_contains_empty_without_empty_literal() {
    let non_empty_literal1 = Literal { content: String::from("hello") };
    let non_empty_literal2 = Literal { content: String::from("world") };
    let set_without_empty = LiteralSet::new(vec![non_empty_literal1, non_empty_literal2]);
    assert!(!set_without_empty.contains_empty());
}

#[test]
fn test_contains_empty_with_multiple_literals_including_empty() {
    let empty_literal = Literal { content: String::new() };
    let non_empty_literal = Literal { content: String::from("test") };
    let set_with_multiple = LiteralSet::new(vec![non_empty_literal, empty_literal]);
    assert!(set_with_multiple.contains_empty());
}

#[test]
fn test_contains_empty_with_only_empty_literals() {
    let empty_literal1 = Literal { content: String::new() };
    let empty_literal2 = Literal { content: String::new() };
    let set_with_only_empty = LiteralSet::new(vec![empty_literal1, empty_literal2]);
    assert!(set_with_only_empty.contains_empty());
}

#[test]
fn test_contains_empty_with_no_literals() {
    let empty_set = LiteralSet::new(vec![]);
    assert!(!empty_set.contains_empty()); // No literals to check against
}


