// Answer 0

#[derive(Debug)]
struct LiteralSet {
    lits: Vec<Vec<u8>>,
}

impl LiteralSet {
    pub fn longest_common_prefix(&self) -> &[u8] {
        if self.lits.is_empty() {
            return &[];
        }
        let lit0 = &self.lits[0];
        let mut len = lit0.len();
        for lit in &self.lits[1..] {
            len = std::cmp::min(
                len,
                lit.iter()
                    .zip(lit0)
                    .take_while(|&(a, b)| a == b)
                    .count(),
            );
        }
        &self.lits[0][..len]
    }
}

#[test]
fn test_longest_common_prefix_single_literal() {
    let set = LiteralSet {
        lits: vec![b"rust".to_vec()],
    };
    assert_eq!(set.longest_common_prefix(), b"rust");
}

#[test]
fn test_longest_common_prefix_multiple_matching_literals() {
    let set = LiteralSet {
        lits: vec![b"rustacean".to_vec(), b"rusty".to_vec(), b"rust".to_vec()],
    };
    assert_eq!(set.longest_common_prefix(), b"rust");
}

#[test]
fn test_longest_common_prefix_no_common_prefix() {
    let set = LiteralSet {
        lits: vec![b"hello".to_vec(), b"world".to_vec()],
    };
    assert_eq!(set.longest_common_prefix(), b""); // No common prefix
}

#[test]
fn test_longest_common_prefix_empty_middle() {
    let set = LiteralSet {
        lits: vec![b"test".to_vec(), b"".to_vec(), b"testimony".to_vec()],
    };
    assert_eq!(set.longest_common_prefix(), b""); // No common prefix due to empty string
}

#[test]
#[should_panic]
fn test_longest_common_prefix_empty_set() {
    let set = LiteralSet {
        lits: vec![],
    };
    set.longest_common_prefix(); // Should panic due to is_empty returning true
}

