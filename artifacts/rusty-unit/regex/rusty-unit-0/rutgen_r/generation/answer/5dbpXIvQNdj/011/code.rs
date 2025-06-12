// Answer 0

#[test]
fn test_cross_add_with_constraints() {
    use std::cmp;

    struct Literal {
        data: Vec<u8>,
        cut: bool,
    }

    impl Literal {
        fn new(data: Vec<u8>) -> Self {
            Literal { data, cut: false }
        }

        fn extend(&mut self, bytes: &[u8]) {
            self.data.extend_from_slice(bytes);
        }

        fn cut(&mut self) {
            self.cut = true;
        }

        fn is_cut(&self) -> bool {
            self.cut
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
            self.lits.iter().map(|lit| lit.data.len()).sum()
        }

        fn cross_add(&mut self, bytes: &[u8]) -> bool {
            if bytes.is_empty() {
                return true;
            }
            if self.lits.is_empty() {
                let i = cmp::min(self.limit_size, bytes.len());
                self.lits.push(Literal::new(bytes[..i].to_owned()));
                self.lits[0].cut = i < bytes.len();
                return !self.lits[0].is_cut();
            }
            let size = self.num_bytes();
            if size + self.lits.len() >= self.limit_size {
                return false;
            }
            let mut i = 1;
            while size + (i * self.lits.len()) <= self.limit_size && i < bytes.len() {
                i += 1;
            }
            for lit in &mut self.lits {
                if !lit.is_cut() {
                    lit.extend(&bytes[..i]);
                    if i < bytes.len() {
                        lit.cut();
                    }
                }
            }
            true
        }
    }

    // Setup test case
    let limit_size = 10;
    let mut set = LiteralSet::new(limit_size);
    
    // Add one literal to the set
    set.lits.push(Literal::new(vec![1, 2]));

    // Set a valid bytes input
    let bytes = vec![3, 4, 5, 6]; // 4 bytes

    // Here, size is 2 (from the existing lit), lits.len() is 1 (one lit in set)
    // Therefore size + lits.len() = 3 which is < limit_size (10)
    // We will iterate to determine valid 'i'
    // `i` should become equal to 'bytes.len()' resulting in valid expansion
    assert_eq!(set.cross_add(&bytes), true);
    assert_eq!(set.lits[0].data, vec![1, 2, 3, 4, 5, 6]);
    assert!(set.lits[0].is_cut());
}

