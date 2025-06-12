// Answer 0

#[test]
fn test_cross_add_non_empty_lits_with_fitting_bytes() {
    struct LocalLiteral {
        v: Vec<u8>,
        cut: bool,
    }

    impl LocalLiteral {
        pub fn new(bytes: Vec<u8>) -> LocalLiteral {
            LocalLiteral { v: bytes, cut: false }
        }
        pub fn is_cut(&self) -> bool {
            self.cut
        }
        pub fn extend(&mut self, bytes: &[u8]) {
            self.v.extend_from_slice(bytes);
        }
        pub fn cut(&mut self) {
            self.cut = true;
        }
    }

    struct LocalLiterals {
        lits: Vec<LocalLiteral>,
        limit_size: usize,
    }

    impl LocalLiterals {
        pub fn new(limit_size: usize) -> LocalLiterals {
            LocalLiterals {
                lits: Vec::new(),
                limit_size,
            }
        }
        
        pub fn num_bytes(&self) -> usize {
            self.lits.iter().map(|lit| lit.v.len()).sum()
        }

        pub fn cross_add(&mut self, bytes: &[u8]) -> bool {
            if bytes.is_empty() {
                return true;
            }
            if self.lits.is_empty() {
                let i = cmp::min(self.limit_size, bytes.len());
                self.lits.push(LocalLiteral::new(bytes[..i].to_owned()));
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

    let mut literals = LocalLiterals::new(10);
    literals.lits.push(LocalLiteral::new(vec![1, 2, 3])); // non-empty lits
    literals.lits.push(LocalLiteral::new(vec![4, 5])); // non-empty lits

    let result = literals.cross_add(&[6, 7, 8]); // fitting bytes
    assert!(result);
    assert_eq!(literals.lits[0].v, vec![1, 2, 3, 6]); // first literal extended
    assert_eq!(literals.lits[1].v, vec![4, 5, 6]); // second literal cut since more bytes available
}

