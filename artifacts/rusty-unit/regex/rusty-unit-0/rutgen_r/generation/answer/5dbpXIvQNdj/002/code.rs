// Answer 0

#[test]
fn test_cross_add_non_empty_bytes_empty_literals() {
    struct Literal {
        data: Vec<u8>,
        cut: bool,
    }

    impl Literal {
        fn new(data: Vec<u8>) -> Self {
            Self { data, cut: false }
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
            Self { lits: Vec::new(), limit_size }
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

        fn num_bytes(&self) -> usize {
            self.lits.iter().map(|lit| lit.data.len()).sum()
        }
    }

    // Test case: Adding bytes when literals set is empty
    let mut set = LiteralSet::new(10);
    let bytes: &[u8] = b"hello";
    let result = set.cross_add(bytes);
    assert!(result); // Expected to return true
    assert!(!set.lits[0].is_cut()); // Expected to return false for is_cut
    assert_eq!(set.lits[0].data, b"hello"); // Ensure data is correct
}

#[test]
fn test_cross_add_bytes_exceed_limit() {
    struct Literal {
        data: Vec<u8>,
        cut: bool,
    }

    impl Literal {
        fn new(data: Vec<u8>) -> Self {
            Self { data, cut: false }
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
            Self { lits: Vec::new(), limit_size }
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

        fn num_bytes(&self) -> usize {
            self.lits.iter().map(|lit| lit.data.len()).sum()
        }
    }

    // Test case: Adding bytes that would exceed the limit size
    let mut set = LiteralSet::new(5);
    let bytes: &[u8] = b"hello world";
    let result = set.cross_add(bytes);
    assert!(!result); // Expected to return false since it exceeds limit
}

