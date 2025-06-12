// Answer 0

#[derive(Default)]
struct Literal {
    is_empty: bool,
}

impl Literal {
    fn new(is_empty: bool) -> Self {
        Self { is_empty }
    }

    fn is_empty(&self) -> bool {
        self.is_empty
    }
}

struct LiteralSet {
    lits: Vec<Literal>,
}

impl LiteralSet {
    fn new(lits: Vec<Literal>) -> Self {
        Self { lits }
    }

    pub fn is_empty(&self) -> bool {
        self.lits.is_empty() || self.lits.iter().all(|lit| lit.is_empty())
    }
}

#[test]
fn test_is_empty_with_empty_set() {
    let empty_set = LiteralSet::new(vec![]);
    assert!(empty_set.is_empty());
}

#[test]
fn test_is_empty_with_non_empty_set() {
    let non_empty_set = LiteralSet::new(vec![Literal::new(false), Literal::new(false)]);
    assert!(!non_empty_set.is_empty());
}

#[test]
fn test_is_empty_with_all_empty_literals() {
    let all_empty_literals = LiteralSet::new(vec![Literal::new(true), Literal::new(true)]);
    assert!(all_empty_literals.is_empty());
}

#[test]
fn test_is_empty_with_mixed_literals() {
    let mixed_literals = LiteralSet::new(vec![Literal::new(false), Literal::new(true)]);
    assert!(!mixed_literals.is_empty());
}

