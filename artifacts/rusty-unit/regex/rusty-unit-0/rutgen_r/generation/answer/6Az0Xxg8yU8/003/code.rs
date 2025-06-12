// Answer 0

#[derive(Debug)]
struct Literal {
    // Assuming Literal has a field for demonstration purposes
    size: usize,
}

impl Literal {
    fn empty() -> Self {
        Literal { size: 0 }
    }
}

#[derive(Debug)]
struct Literals {
    lits: Vec<Literal>,
}

impl Literals {
    fn new() -> Self {
        Literals { lits: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.lits.is_empty()
    }

    fn num_bytes(&self) -> usize {
        self.lits.iter().map(|lit| lit.size).sum()
    }

    fn add_literal(&mut self, literal: Literal) {
        self.lits.push(literal);
    }
}

#[derive(Debug)]
struct LiteralSet {
    lits: Vec<Literal>,
    limit_size: usize,
}

impl LiteralSet {
    fn new(limit_size: usize) -> Self {
        LiteralSet {
            lits: Vec::new(),
            limit_size,
        }
    }

    fn num_bytes(&self) -> usize {
        self.lits.iter().map(|lit| lit.size).sum()
    }

    pub fn union(&mut self, lits: Literals) -> bool {
        if self.num_bytes() + lits.num_bytes() > self.limit_size {
            return false;
        }
        if lits.is_empty() {
            self.lits.push(Literal::empty());
        } else {
            self.lits.extend(lits.lits);
        }
        true
    }
}

#[test]
fn test_union_with_limit_edge_case() {
    let limit_size = 10;
    let mut set = LiteralSet::new(limit_size);

    let mut lits = Literals::new();
    lits.add_literal(Literal { size: 5 });
    lits.add_literal(Literal { size: 5 });

    assert_eq!(set.union(lits), true);
}

#[test]
fn test_union_with_non_empty_literals() {
    let limit_size = 20;
    let mut set = LiteralSet::new(limit_size);

    let mut lits = Literals::new();
    lits.add_literal(Literal { size: 8 });
    lits.add_literal(Literal { size: 4 });

    assert_eq!(set.union(lits), true);
}

