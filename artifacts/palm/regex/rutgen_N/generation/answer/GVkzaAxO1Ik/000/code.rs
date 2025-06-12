// Answer 0

#[derive(Debug)]
struct Literal {
    is_cut: bool,
}

impl Literal {
    pub fn new() -> Self {
        Literal { is_cut: false }
    }

    pub fn cut(&mut self) {
        self.is_cut = true;
    }
}

#[derive(Debug)]
struct LiteralSet {
    lits: Vec<Literal>,
}

impl LiteralSet {
    pub fn new() -> Self {
        LiteralSet { lits: Vec::new() }
    }

    pub fn add_literal(&mut self, literal: Literal) {
        self.lits.push(literal);
    }

    pub fn cut(&mut self) {
        for lit in &mut self.lits {
            lit.cut();
        }
    }
}

#[test]
fn test_cut_no_literals() {
    let mut set = LiteralSet::new();
    set.cut();
    assert!(set.lits.is_empty());
}

#[test]
fn test_cut_single_literal() {
    let mut set = LiteralSet::new();
    set.add_literal(Literal::new());
    set.cut();
    assert!(set.lits[0].is_cut);
}

#[test]
fn test_cut_multiple_literals() {
    let mut set = LiteralSet::new();
    set.add_literal(Literal::new());
    set.add_literal(Literal::new());
    set.cut();
    assert!(set.lits[0].is_cut);
    assert!(set.lits[1].is_cut);
}

