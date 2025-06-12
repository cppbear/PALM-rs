// Answer 0

#[derive(Default)]
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

    fn num_bytes(&self) -> usize {
        self.lits.iter().map(|lit| lit.data.len()).sum()
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

#[test]
fn test_cross_add_with_full_conditions() {
    let mut lit_set = LiteralSet::new(10);
    lit_set.lits.push(Literal::new(vec![1, 2]));
    
    let bytes_to_add = vec![3, 4, 5];
    assert_eq!(lit_set.cross_add(&bytes_to_add), true);
    assert!(!lit_set.lits[0].is_cut());
    assert_eq!(lit_set.lits[0].data, vec![1, 2, 3]);
}

#[test]
fn test_cross_add_with_cutting() {
    let mut lit_set = LiteralSet::new(5);
    lit_set.lits.push(Literal::new(vec![1]));

    let bytes_to_add = vec![2, 3, 4, 5];
    assert_eq!(lit_set.cross_add(&bytes_to_add), true);
    assert!(lit_set.lits[0].is_cut());
    assert_eq!(lit_set.lits[0].data, vec![1, 2]);
}

#[test]
fn test_cross_add_with_limit_reached() {
    let mut lit_set = LiteralSet::new(3);
    lit_set.lits.push(Literal::new(vec![1, 2]));
    
    let bytes_to_add = vec![3, 4];
    assert_eq!(lit_set.cross_add(&bytes_to_add), false);
    assert_eq!(lit_set.lits[0].data, vec![1, 2]);
}

#[test]
fn test_cross_add_empty_lits() {
    let mut lit_set = LiteralSet::new(10);
    
    let bytes_to_add = vec![1, 2, 3];
    assert_eq!(lit_set.cross_add(&bytes_to_add), true);
    assert_eq!(lit_set.lits[0].data, vec![1, 2, 3]);
}

#[test]
fn test_cross_add_empty_bytes() {
    let mut lit_set = LiteralSet::new(10);
    lit_set.lits.push(Literal::new(vec![1]));

    let bytes_to_add = vec![];
    assert_eq!(lit_set.cross_add(&bytes_to_add), true);
}

