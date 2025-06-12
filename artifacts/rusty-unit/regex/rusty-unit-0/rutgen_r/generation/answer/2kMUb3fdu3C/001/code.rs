// Answer 0

#[test]
fn test_add_literal_exceeds_limit() {
    struct Literal {
        len: usize,
    }

    struct LiteralSet {
        lits: Vec<Literal>,
        limit_size: usize,
    }

    impl LiteralSet {
        pub fn new(limit_size: usize) -> Self {
            LiteralSet {
                lits: Vec::new(),
                limit_size,
            }
        }

        pub fn num_bytes(&self) -> usize {
            self.lits.iter().map(|lit| lit.len).sum()
        }

        pub fn add(&mut self, lit: Literal) -> bool {
            if self.num_bytes() + lit.len > self.limit_size {
                return false;
            }
            self.lits.push(lit);
            true
        }
    }

    let limit_size = 10;
    let mut set = LiteralSet::new(limit_size);
    
    set.add(Literal { len: 5 }); // Within limit
    let result = set.add(Literal { len: 6 }); // Exceeds limit
    assert!(!result, "Expected to return false when adding literal exceeding limit size");
}

