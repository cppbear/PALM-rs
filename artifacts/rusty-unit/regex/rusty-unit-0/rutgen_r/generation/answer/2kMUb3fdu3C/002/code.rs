// Answer 0

#[test]
fn test_add_literal_at_limit_size() {
    struct Literal {
        len: usize,
    }

    impl Literal {
        fn new(len: usize) -> Self {
            Literal { len }
        }

        fn len(&self) -> usize {
            self.len
        }
    }

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
            self.lits.iter().map(|lit| lit.len()).sum()
        }

        pub fn add(&mut self, lit: Literal) -> bool {
            if self.num_bytes() + lit.len() > self.limit_size {
                return false;
            }
            self.lits.push(lit);
            true
        }
    }

    let limit_size = 10;
    let mut literal_set = LiteralSet::new(limit_size);
    let lit = Literal::new(10); // exactly at the limit

    let result = literal_set.add(lit);
    assert!(result); // should return true
}

