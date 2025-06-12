// Answer 0


struct Literal {
    is_empty: bool,
}

impl Literal {
    fn new(is_empty: bool) -> Self {
        Literal { is_empty }
    }
    
    fn is_empty(&self) -> bool {
        self.is_empty
    }
}

struct Set {
    lits: Vec<Literal>,
}

impl Set {
    fn new(lits: Vec<Literal>) -> Self {
        Set { lits }
    }
    
    fn is_empty(&self) -> bool {
        self.lits.is_empty() || self.lits.iter().all(|lit| lit.is_empty())
    }
}

#[test]
fn test_set_with_non_empty_literals() {
    let literals = vec![Literal::new(false), Literal::new(false)];
    let set = Set::new(literals);
    assert!(!set.is_empty());
}

#[test]
fn test_set_with_one_empty_literal() {
    let literals = vec![Literal::new(true), Literal::new(false)];
    let set = Set::new(literals);
    assert!(!set.is_empty());
}

#[test]
fn test_set_with_all_empty_literals() {
    let literals = vec![Literal::new(true), Literal::new(true)];
    let set = Set::new(literals);
    assert!(set.is_empty());
}

#[test]
fn test_set_with_empty_literal_and_non_empty() {
    let literals = vec![Literal::new(false), Literal::new(true)];
    let set = Set::new(literals);
    assert!(!set.is_empty());
}


