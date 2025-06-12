// Answer 0

#[derive(Debug, PartialEq)]
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
fn test_remove_complete_all_non_cut_literals() {
    let mut set = LiteralSet::new(vec![
        Literal { cut: false },
        Literal { cut: false },
        Literal { cut: false },
    ]);
    let result = set.remove_complete();
    assert_eq!(result, set.lits);
}

#[test]
fn test_remove_complete_mixed_literals() {
    let mut set = LiteralSet::new(vec![
        Literal { cut: true },
        Literal { cut: false },
        Literal { cut: true },
        Literal { cut: false },
    ]);
    let result = set.remove_complete();
    assert_eq!(result, vec![
        Literal { cut: false },
        Literal { cut: false },
    ]);
    assert_eq!(set.lits, vec![
        Literal { cut: true },
        Literal { cut: true },
    ]);
}

#[test]
fn test_remove_complete_all_cut_literals() {
    let mut set = LiteralSet::new(vec![
        Literal { cut: true },
        Literal { cut: true },
    ]);
    let result = set.remove_complete();
    assert_eq!(result, vec![]);
    assert_eq!(set.lits, vec![
        Literal { cut: true },
        Literal { cut: true },
    ]);
}

