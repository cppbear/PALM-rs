// Answer 0

#[derive(Debug)]
struct LiteralSet {
    lits: Vec<Vec<u8>>,
}

impl LiteralSet {
    fn new(lits: Vec<Vec<u8>>) -> Self {
        Self { lits }
    }
    
    fn is_empty(&self) -> bool {
        self.lits.is_empty()
    }
    
    pub fn longest_common_suffix(&self) -> &[u8] {
        if self.is_empty() {
            return &[];
        }
        let lit0 = &*self.lits[0];
        let mut len = lit0.len();
        for lit in &self.lits[1..] {
            len = std::cmp::min(
                len,
                lit.iter()
                   .rev()
                   .zip(lit0.iter().rev())
                   .take_while(|&(a, b)| a == b)
                   .count());
        }
        &self.lits[0][self.lits[0].len() - len..]
    }
}

#[test]
fn test_longest_common_suffix_with_common_suffix() {
    let set = LiteralSet::new(vec![b"abcde".to_vec(), b"fabcde".to_vec(), b"gabcde".to_vec()]);
    let result = set.longest_common_suffix();
    assert_eq!(result, b"cde");
}

#[test]
fn test_longest_common_suffix_with_no_common_suffix() {
    let set = LiteralSet::new(vec![b"abc".to_vec(), b"def".to_vec()]);
    let result = set.longest_common_suffix();
    assert_eq!(result, b"");
}

#[test]
fn test_longest_common_suffix_with_empty_set() {
    let set: LiteralSet = LiteralSet::new(vec![]);
    let result = set.longest_common_suffix();
    assert_eq!(result, b"");
}

#[test]
fn test_longest_common_suffix_with_single_literal() {
    let set = LiteralSet::new(vec![b"xyz".to_vec()]);
    let result = set.longest_common_suffix();
    assert_eq!(result, b"xyz");
}

#[test]
fn test_longest_common_suffix_with_multiple_identical_literals() {
    let set = LiteralSet::new(vec![b"abc".to_vec(), b"abc".to_vec(), b"abc".to_vec()]);
    let result = set.longest_common_suffix();
    assert_eq!(result, b"abc");
}

