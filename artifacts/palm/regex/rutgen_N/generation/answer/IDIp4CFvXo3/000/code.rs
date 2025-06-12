// Answer 0

#[derive(Debug)]
struct Literal {
    cut: bool,
}

impl Literal {
    fn is_cut(&self) -> bool {
        self.cut
    }
}

struct LiteralSet {
    lits: Vec<Literal>,
}

impl LiteralSet {
    pub fn any_complete(&self) -> bool {
        self.lits.iter().any(|lit| !lit.is_cut())
    }
}

#[test]
fn test_any_complete_with_complete_literals() {
    let set = LiteralSet {
        lits: vec![Literal { cut: true }, Literal { cut: true }],
    };
    assert_eq!(set.any_complete(), false);
}

#[test]
fn test_any_complete_with_incomplete_literal() {
    let set = LiteralSet {
        lits: vec![Literal { cut: true }, Literal { cut: false }],
    };
    assert_eq!(set.any_complete(), true);
}

#[test]
fn test_any_complete_with_all_incomplete_literals() {
    let set = LiteralSet {
        lits: vec![Literal { cut: false }, Literal { cut: false }],
    };
    assert_eq!(set.any_complete(), true);
}

#[test]
fn test_any_complete_with_empty_set() {
    let set = LiteralSet { lits: vec![] };
    assert_eq!(set.any_complete(), false);
}

