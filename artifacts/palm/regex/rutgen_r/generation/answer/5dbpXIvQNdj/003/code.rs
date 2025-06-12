// Answer 0

#[derive(Default)]
struct Literal {
    bytes: Vec<u8>,
    cut: bool,
}

impl Literal {
    fn new(bytes: Vec<u8>) -> Self {
        Self { bytes, cut: false }
    }
    
    fn extend(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }
    
    fn is_cut(&self) -> bool {
        self.cut
    }
    
    fn cut(&mut self) {
        self.cut = true;
    }
}

struct LiteralSet {
    lits: Vec<Literal>,
    limit_size: usize,
}

impl LiteralSet {
    fn new(limit_size: usize) -> Self {
        Self {
            lits: Vec::new(),
            limit_size,
        }
    }
    
    fn num_bytes(&self) -> usize {
        self.lits.iter().map(|lit| lit.bytes.len()).sum()
    }
    
    pub fn cross_add(&mut self, bytes: &[u8]) -> bool {
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

#[test]
fn test_cross_add_exceeds_limit() {
    let mut set = LiteralSet::new(5);
    set.lits.push(Literal::new(vec![1])); // Initial size is 1
    set.lits.push(Literal::new(vec![2])); // Current size becomes 2

    let bytes = vec![3, 4]; // Trying to add 2 bytes
    // The current size (2) + number of literals (2) >= limit size (5)
    assert_eq!(set.cross_add(&bytes), false);
}

#[test]
fn test_cross_add_with_cutting() {
    let mut set = LiteralSet::new(5);
    set.lits.push(Literal::new(vec![1])); // Initial size is 1
    set.lits.push(Literal::new(vec![2])); // Current size becomes 2

    let bytes = vec![3, 4, 5]; // Trying to add 3 bytes
    // The current size (2) + number of literals (2) = limit size (5)
    assert_eq!(set.cross_add(&bytes), false);
}

