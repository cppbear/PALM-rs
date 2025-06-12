// Answer 0

#[test]
fn test_cross_add_non_empty_bytes_and_non_empty_lits_with_limit() {
    struct Literal {
        bytes: Vec<u8>,
        cut: bool,
    }

    impl Literal {
        fn new(bytes: Vec<u8>) -> Self {
            Self { bytes, cut: false }
        }

        fn extend(&mut self, other: &[u8]) {
            self.bytes.extend_from_slice(other);
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
            Self { lits: Vec::new(), limit_size }
        }

        fn num_bytes(&self) -> usize {
            self.lits.iter().map(|lit| lit.bytes.len()).sum()
        }

        fn cross_add(&mut self, bytes: &[u8]) -> bool {
            if bytes.is_empty() {
                return true;
            }
            if self.lits.is_empty() {
                let i = std::cmp::min(self.limit_size, bytes.len());
                self.lits.push(Literal::new(bytes[..i].to_owned()));
                self.lits[0].cut = i < bytes.len();
                return !self.lits[0].is_cut();
            }
            let size = self.num_bytes();
            if size + self.lits.len() >= self.limit_size {
                return false;
            }
            let mut i = 1;
            while size + (i * self.lits.len()) <= self.limit_size
                && i < bytes.len() {
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

    let mut lit_set = LiteralSet::new(10);
    lit_set.lits.push(Literal::new(vec![1, 2, 3]));

    // Setting up the test case with constraints.
    let bytes = vec![4, 5, 6]; // bytes.len() = 3
    let result = lit_set.cross_add(&bytes);

    assert!(result);
    assert_eq!(lit_set.lits.len(), 1);
    assert_eq!(lit_set.lits[0].bytes, vec![1, 2, 3, 4, 5, 6]); // maximum extention
    assert!(!lit_set.lits[0].is_cut()); // should not be cut
}

#[test]
fn test_cross_add_with_limit_panic_condition() {
    struct Literal {
        bytes: Vec<u8>,
        cut: bool,
    }

    impl Literal {
        fn new(bytes: Vec<u8>) -> Self {
            Self { bytes, cut: false }
        }

        fn extend(&mut self, other: &[u8]) {
            self.bytes.extend_from_slice(other);
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
            Self { lits: Vec::new(), limit_size }
        }

        fn num_bytes(&self) -> usize {
            self.lits.iter().map(|lit| lit.bytes.len()).sum()
        }

        fn cross_add(&mut self, bytes: &[u8]) -> bool {
            if bytes.is_empty() {
                return true;
            }
            if self.lits.is_empty() {
                let i = std::cmp::min(self.limit_size, bytes.len());
                self.lits.push(Literal::new(bytes[..i].to_owned()));
                self.lits[0].cut = i < bytes.len();
                return !self.lits[0].is_cut();
            }
            let size = self.num_bytes();
            if size + self.lits.len() >= self.limit_size {
                return false;
            }
            let mut i = 1;
            while size + (i * self.lits.len()) <= self.limit_size
                && i < bytes.len() {
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

    let mut lit_set = LiteralSet::new(5);
    lit_set.lits.push(Literal::new(vec![1, 2]));

    // Setting up the test case to hit the panic condition
    let bytes = vec![3, 4, 5, 6]; // i should become equal to bytes.len() and thus should trigger panic

    let result = lit_set.cross_add(&bytes);

    assert!(result);
    assert_eq!(lit_set.lits.len(), 1);
    assert_eq!(lit_set.lits[0].bytes, vec![1, 2, 3, 4]); // maximum extension with cut
    assert!(lit_set.lits[0].is_cut()); // should be cut
}

