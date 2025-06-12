// Answer 0

#[test]
fn test_union_success_empty() {
    struct Literals {
        lits: Vec<Literal>,
    }

    impl Literals {
        fn new(lits: Vec<Literal>) -> Self {
            Self { lits }
        }

        fn num_bytes(&self) -> usize {
            self.lits.iter().map(|lit| lit.size()).sum()
        }

        fn is_empty(&self) -> bool {
            self.lits.is_empty()
        }
    }

    struct Literal {
        data: String,
    }

    impl Literal {
        fn empty() -> Self {
            Self { data: String::new() }
        }

        fn size(&self) -> usize {
            self.data.len()
        }
    }

    struct Set {
        lits: Vec<Literal>,
        limit_size: usize,
    }

    impl Set {
        fn new(limit_size: usize) -> Self {
            Self {
                lits: Vec::new(),
                limit_size,
            }
        }

        fn num_bytes(&self) -> usize {
            self.lits.iter().map(|lit| lit.size()).sum()
        }

        fn union(&mut self, lits: Literals) -> bool {
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

    let mut set = Set::new(10);
    let empty_literals = Literals::new(Vec::new());

    assert!(set.union(empty_literals));
    assert_eq!(set.lits.len(), 1);
}

#[test]
fn test_union_success_non_empty() {
    struct Literals {
        lits: Vec<Literal>,
    }

    impl Literals {
        fn new(lits: Vec<Literal>) -> Self {
            Self { lits }
        }

        fn num_bytes(&self) -> usize {
            self.lits.iter().map(|lit| lit.size()).sum()
        }

        fn is_empty(&self) -> bool {
            self.lits.is_empty()
        }
    }

    struct Literal {
        data: String,
    }

    impl Literal {
        fn new(data: &str) -> Self {
            Self { data: data.to_string() }
        }

        fn size(&self) -> usize {
            self.data.len()
        }
    }

    struct Set {
        lits: Vec<Literal>,
        limit_size: usize,
    }

    impl Set {
        fn new(limit_size: usize) -> Self {
            Self {
                lits: Vec::new(),
                limit_size,
            }
        }

        fn num_bytes(&self) -> usize {
            self.lits.iter().map(|lit| lit.size()).sum()
        }

        fn union(&mut self, lits: Literals) -> bool {
            if self.num_bytes() + lits.num_bytes() > self.limit_size {
                return false;
            }
            if lits.is_empty() {
                self.lits.push(Literal::new(""));
            } else {
                self.lits.extend(lits.lits);
            }
            true
        }
    }

    let mut set = Set::new(20);
    let literals = Literals::new(vec![Literal::new("a"), Literal::new("b")]);

    assert!(set.union(literals));
    assert_eq!(set.lits.len(), 2);
}

#[test]
fn test_union_exceeds_limit() {
    struct Literals {
        lits: Vec<Literal>,
    }

    impl Literals {
        fn new(lits: Vec<Literal>) -> Self {
            Self { lits }
        }

        fn num_bytes(&self) -> usize {
            self.lits.iter().map(|lit| lit.size()).sum()
        }

        fn is_empty(&self) -> bool {
            self.lits.is_empty()
        }
    }

    struct Literal {
        data: String,
    }

    impl Literal {
        fn new(data: &str) -> Self {
            Self { data: data.to_string() }
        }

        fn size(&self) -> usize {
            self.data.len()
        }
    }

    struct Set {
        lits: Vec<Literal>,
        limit_size: usize,
    }

    impl Set {
        fn new(limit_size: usize) -> Self {
            Self {
                lits: Vec::new(),
                limit_size,
            }
        }

        fn num_bytes(&self) -> usize {
            self.lits.iter().map(|lit| lit.size()).sum()
        }

        fn union(&mut self, lits: Literals) -> bool {
            if self.num_bytes() + lits.num_bytes() > self.limit_size {
                return false;
            }
            if lits.is_empty() {
                self.lits.push(Literal::new(""));
            } else {
                self.lits.extend(lits.lits);
            }
            true
        }
    }

    let mut set = Set::new(3);
    let literals = Literals::new(vec![Literal::new("abcd")]);

    assert!(!set.union(literals));
    assert_eq!(set.lits.len(), 0);
}

