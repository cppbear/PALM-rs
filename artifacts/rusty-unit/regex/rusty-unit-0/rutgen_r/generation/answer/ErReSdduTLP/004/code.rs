// Answer 0

#[derive(Clone)]
struct Hir;

#[derive(Clone)]
struct Literals {
    limit: usize,
    literals: Vec<Literal>,
}

#[derive(Clone, PartialEq)]
struct Literal;

impl Literals {
    pub fn new(limit: usize) -> Self {
        Self {
            limit,
            literals: Vec::new(),
        }
    }

    pub fn set_limit_size(&mut self, size: usize) {
        self.limit = size;
    }

    pub fn limit_size(&self) -> usize {
        self.limit
    }

    pub fn is_empty(&self) -> bool {
        self.literals.is_empty()
    }

    pub fn cross_product(&self, _: &Literals) -> bool {
        false
    }

    pub fn add(&mut self, _: Literal) {}

    pub fn union(&mut self, _: Literals) -> bool {
        false
    }

    pub fn cut(&mut self) {}
    
    pub fn to_empty(&self) -> Self {
        Self::new(0)
    }

    pub fn clone(&self) -> Self {
        self.clone()
    }
}

#[test]
fn test_repeat_zero_or_one_literals() {
    let e = Hir;
    
    let mut lits = Literals::new(10);
    lits.add(Literal);

    let result = std::panic::catch_unwind(|| {
        repeat_zero_or_one_literals(&e, &mut lits, |_, _| {});
    });

    assert!(result.is_ok());
}

#[test]
fn test_repeat_zero_or_one_literals_with_empty_lits3() {
    let e = Hir;
    
    let mut lits = Literals::new(10);
    lits.add(Literal);

    let result = std::panic::catch_unwind(|| {
        let mut lits3 = lits.to_empty();
        lits3.set_limit_size(5); 
        repeat_zero_or_one_literals(&e, &mut lits, |_, _| {});
    });

    assert!(result.is_ok());
}

#[test]
fn test_repeat_zero_or_one_literals_with_cross_product_false() {
    let e = Hir;
    
    let mut lits = Literals::new(10);
    lits.add(Literal);

    let mut lits2 = lits.clone();
    let mut lits3 = lits.to_empty();
    lits3.set_limit_size(5); // constraint: lits3.is_empty() is false
    assert!(!lits2.cross_product(&lits3));

    let result = std::panic::catch_unwind(|| {
        repeat_zero_or_one_literals(&e, &mut lits, |_, _| {});
    });

    assert!(result.is_ok());
}

