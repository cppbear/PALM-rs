// Answer 0

#[derive(Debug)]
struct Literal {
    is_cut: bool,
}

impl Literal {
    fn new(is_cut: bool) -> Self {
        Literal { is_cut }
    }

    fn is_cut(&self) -> bool {
        self.is_cut
    }
}

#[derive(Debug)]
struct LiteralSet {
    lits: Vec<Literal>,
}

impl LiteralSet {
    fn new(lits: Vec<Literal>) -> Self {
        LiteralSet { lits }
    }

    pub fn all_complete(&self) -> bool {
        !self.lits.is_empty() && self.lits.iter().all(|l| !l.is_cut())
    }
}

#[test]
fn test_all_complete_with_non_empty_set_no_cuts() {
    let literals = vec![Literal::new(false), Literal::new(false)];
    let set = LiteralSet::new(literals);
    assert!(set.all_complete());
}

#[test]
fn test_all_complete_with_non_empty_set_with_cuts() {
    let literals = vec![Literal::new(false), Literal::new(true)];
    let set = LiteralSet::new(literals);
    assert!(!set.all_complete());
}

#[test]
fn test_all_complete_with_empty_set() {
    let set = LiteralSet::new(vec![]);
    assert!(!set.all_complete());
}

#[test]
fn test_all_complete_with_single_literal_no_cut() {
    let literals = vec![Literal::new(false)];
    let set = LiteralSet::new(literals);
    assert!(set.all_complete());
}

#[test]
fn test_all_complete_with_single_literal_with_cut() {
    let literals = vec![Literal::new(true)];
    let set = LiteralSet::new(literals);
    assert!(!set.all_complete());
}

