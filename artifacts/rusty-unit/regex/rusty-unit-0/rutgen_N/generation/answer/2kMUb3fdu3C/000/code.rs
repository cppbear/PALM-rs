// Answer 0

#[test]
fn test_add_literal_within_limit() {
    struct Literal {
        value: String,
    }

    struct LiteralSet {
        lits: Vec<Literal>,
        limit_size: usize,
    }

    impl LiteralSet {
        pub fn new(limit_size: usize) -> Self {
            Self {
                lits: Vec::new(),
                limit_size,
            }
        }

        pub fn num_bytes(&self) -> usize {
            self.lits.iter().map(|lit| lit.value.len()).sum()
        }

        pub fn add(&mut self, lit: Literal) -> bool {
            if self.num_bytes() + lit.value.len() > self.limit_size {
                return false;
            }
            self.lits.push(lit);
            true
        }
    }

    let mut set = LiteralSet::new(10);
    let literal = Literal { value: "a".to_string() };
    assert!(set.add(literal));
}

#[test]
fn test_add_literal_exceeds_limit() {
    struct Literal {
        value: String,
    }

    struct LiteralSet {
        lits: Vec<Literal>,
        limit_size: usize,
    }

    impl LiteralSet {
        pub fn new(limit_size: usize) -> Self {
            Self {
                lits: Vec::new(),
                limit_size,
            }
        }

        pub fn num_bytes(&self) -> usize {
            self.lits.iter().map(|lit| lit.value.len()).sum()
        }

        pub fn add(&mut self, lit: Literal) -> bool {
            if self.num_bytes() + lit.value.len() > self.limit_size {
                return false;
            }
            self.lits.push(lit);
            true
        }
    }

    let mut set = LiteralSet::new(5);
    let literal1 = Literal { value: "abc".to_string() };
    let literal2 = Literal { value: "de".to_string() };
    assert!(set.add(literal1));
    assert!(!set.add(literal2));
}

#[test]
fn test_add_literal_at_limit() {
    struct Literal {
        value: String,
    }

    struct LiteralSet {
        lits: Vec<Literal>,
        limit_size: usize,
    }

    impl LiteralSet {
        pub fn new(limit_size: usize) -> Self {
            Self {
                lits: Vec::new(),
                limit_size,
            }
        }

        pub fn num_bytes(&self) -> usize {
            self.lits.iter().map(|lit| lit.value.len()).sum()
        }

        pub fn add(&mut self, lit: Literal) -> bool {
            if self.num_bytes() + lit.value.len() > self.limit_size {
                return false;
            }
            self.lits.push(lit);
            true
        }
    }

    let mut set = LiteralSet::new(3);
    let literal = Literal { value: "abc".to_string() };
    assert!(set.add(literal));
    assert!(!set.add(Literal { value: "d".to_string() }));
}

