// Answer 0

#[derive(Debug, Clone)]
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

struct LiteralSet {
    lits: Vec<Literal>,
}

impl LiteralSet {
    fn new(lits: Vec<Literal>) -> Self {
        LiteralSet { lits }
    }

    fn remove_complete(&mut self) -> Vec<Literal> {
        let mut base = vec![];
        for lit in std::mem::replace(&mut self.lits, vec![]) {
            if lit.is_cut() {
                self.lits.push(lit);
            } else {
                base.push(lit);
            }
        }
        base
    }
}

#[test]
fn test_remove_complete_no_literals() {
    let mut literal_set = LiteralSet::new(vec![]);
    let removed = literal_set.remove_complete();
    assert_eq!(removed.len(), 0);
    assert_eq!(literal_set.lits.len(), 0);
}

#[test]
fn test_remove_complete_only_cuts() {
    let mut literal_set = LiteralSet::new(vec![Literal::new(true), Literal::new(true)]);
    let removed = literal_set.remove_complete();
    assert_eq!(removed.len(), 0);
    assert_eq!(literal_set.lits.len(), 2);
}

#[test]
fn test_remove_complete_mixed_literals() {
    let mut literal_set = LiteralSet::new(vec![Literal::new(false), Literal::new(true), Literal::new(false)]);
    let removed = literal_set.remove_complete();
    assert_eq!(removed.len(), 2);
    assert_eq!(literal_set.lits.len(), 1);
    assert!(literal_set.lits[0].is_cut());
}

#[test]
fn test_remove_complete_all_non_cut() {
    let mut literal_set = LiteralSet::new(vec![Literal::new(false), Literal::new(false)]);
    let removed = literal_set.remove_complete();
    assert_eq!(removed.len(), 2);
    assert_eq!(literal_set.lits.len(), 0);
}

