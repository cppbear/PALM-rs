// Answer 0

#[derive(Debug)]
struct LitSet<'a> {
    lits: Vec<&'a [u8]>,
}

impl<'a> LitSet<'a> {
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
    
    pub fn is_empty(&self) -> bool {
        self.lits.is_empty()
    }
}

#[test]
fn test_longest_common_suffix_basic() {
    let lit_set = LitSet {
        lits: vec![b"hello", b"cello", b"yellow"],
    };
    assert_eq!(lit_set.longest_common_suffix(), b"lo");
}

#[test]
fn test_longest_common_suffix_no_suffix() {
    let lit_set = LitSet {
        lits: vec![b"hello", b"world", b"rust"],
    };
    assert_eq!(lit_set.longest_common_suffix(), b"");
}

#[test]
fn test_longest_common_suffix_empty_suffix() {
    let lit_set = LitSet {
        lits: vec![b"test", b"test", b"test"],
    };
    assert_eq!(lit_set.longest_common_suffix(), b"test");
}

#[test]
fn test_longest_common_suffix_multiple_suffixes() {
    let lit_set = LitSet {
        lits: vec![b"abc123", b"def123", b"ghi123"],
    };
    assert_eq!(lit_set.longest_common_suffix(), b"123");
}

#[test]
fn test_longest_common_suffix_single_element() {
    let lit_set = LitSet {
        lits: vec![b"single"],
    };
    assert_eq!(lit_set.longest_common_suffix(), b"single");
}

#[test]
#[should_panic]
fn test_longest_common_suffix_empty_set() {
    let lit_set = LitSet {
        lits: vec![],
    };
    lit_set.longest_common_suffix();
}

