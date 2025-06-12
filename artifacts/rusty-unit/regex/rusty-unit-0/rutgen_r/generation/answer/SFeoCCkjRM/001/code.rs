// Answer 0

#[derive(Debug, Clone)]
struct Literal {
    cut: bool,
}

impl Literal {
    fn new(cut: bool) -> Self {
        Literal { cut }
    }

    fn is_cut(&self) -> bool {
        self.cut
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
    let mut set = LiteralSet::new(vec![]);
    let result = set.remove_complete();
    assert_eq!(result, vec![]);
}

#[test]
fn test_remove_complete_all_cuts() {
    let mut set = LiteralSet::new(vec![Literal::new(true), Literal::new(true)]);
    let result = set.remove_complete();
    assert_eq!(result, vec![]);
    assert_eq!(set.lits.len(), 2);
}

#[test]
fn test_remove_complete_mixed_literals() {
    let mut set = LiteralSet::new(vec![
        Literal::new(false),
        Literal::new(true),
        Literal::new(false),
    ]);
    let result = set.remove_complete();
    assert_eq!(result, vec![Literal::new(false), Literal::new(false)]);
    assert_eq!(set.lits.len(), 1);
}

