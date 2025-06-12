// Answer 0

#[test]
fn test_cross_add_empty_set() {
    struct Literal {
        data: Vec<u8>,
        cut: bool,
    }
    
    impl Literal {
        fn new(data: Vec<u8>) -> Self {
            Self { data, cut: false }
        }
        
        fn is_cut(&self) -> bool {
            self.cut
        }
        
        fn cut(&mut self) {
            self.cut = true;
        }
        
        fn extend(&mut self, bytes: &[u8]) {
            self.data.extend_from_slice(bytes);
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
            let size = self.lits.iter().map(|lit| lit.data.len()).sum::<usize>();
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
    
    let mut set = LiteralSet::new(10);
    assert!(set.cross_add(&[1, 2, 3]));
    assert_eq!(set.lits.len(), 1);
    assert_eq!(set.lits[0].data, vec![1, 2, 3]);
    assert!(!set.lits[0].is_cut());
}

#[test]
fn test_cross_add_non_empty_set() {
    struct Literal {
        data: Vec<u8>,
        cut: bool,
    }

    impl Literal {
        fn new(data: Vec<u8>) -> Self {
            Self { data, cut: false }
        }

        fn is_cut(&self) -> bool {
            self.cut
        }

        fn cut(&mut self) {
            self.cut = true;
        }

        fn extend(&mut self, bytes: &[u8]) {
            self.data.extend_from_slice(bytes);
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
            let size = self.lits.iter().map(|lit| lit.data.len()).sum::<usize>();
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

    let mut set = LiteralSet::new(10);
    set.cross_add(&[1, 2, 3]);
    assert!(!set.cross_add(&[4, 5, 6, 7])); // Limit will be exceeded
    assert_eq!(set.lits.len(), 1);
    assert_eq!(set.lits[0].data, vec![1, 2, 3, 4, 5, 6]);
    assert!(set.lits[0].is_cut());
}

#[test]
fn test_cross_add_boundary_condition() {
    struct Literal {
        data: Vec<u8>,
        cut: bool,
    }

    impl Literal {
        fn new(data: Vec<u8>) -> Self {
            Self { data, cut: false }
        }

        fn is_cut(&self) -> bool {
            self.cut
        }

        fn cut(&mut self) {
            self.cut = true;
        }

        fn extend(&mut self, bytes: &[u8]) {
            self.data.extend_from_slice(bytes);
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
            let size = self.lits.iter().map(|lit| lit.data.len()).sum::<usize>();
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
    
    let mut set = LiteralSet::new(5);
    set.cross_add(&[1]);
    assert_eq!(set.lits.len(), 1);
    assert_eq!(set.lits[0].data, vec![1]);
    assert!(set.cross_add(&[2])); // Fits, total will be 2
    assert!(!set.cross_add(&[3])); // Exceeds limit
}

