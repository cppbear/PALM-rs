// Answer 0

#[derive(Debug)]
struct LitSet {
    lits: Vec<Vec<u8>>,
}

impl LitSet {
    pub fn is_empty(&self) -> bool {
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
fn test_longest_common_suffix_non_empty() {
    let set = LitSet {
        lits: vec![
            vec![b'a', b'b', b'c', b'd'], 
            vec![b'a', b'b', b'c', b'e'], 
            vec![b'a', b'b', b'c', b'f'],
        ],
    };
    let result = set.longest_common_suffix();
    assert_eq!(result, b"c");
}

#[test]
fn test_longest_common_suffix_identical() {
    let set = LitSet {
        lits: vec![
            vec![b'x', b'y', b'z'],
            vec![b'x', b'y', b'z'],
            vec![b'x', b'y', b'z'],
        ],
    };
    let result = set.longest_common_suffix();
    assert_eq!(result, b"x");
}

#[test]
fn test_longest_common_suffix_empty_suffix() {
    let set = LitSet {
        lits: vec![
            vec![b'k', b'l', b'm'],
            vec![b'n', b'o', b'p'],
        ],
    };
    let result = set.longest_common_suffix();
    assert_eq!(result, b"");
}

#[should_panic]
fn test_longest_common_suffix_panic_empty_set() {
    let set = LitSet { lits: vec![] };
    let _ = set.longest_common_suffix();
}

#[should_panic]
fn test_longest_common_suffix_panic_index_out_of_bounds() {
    let set = LitSet {
        lits: vec![vec![b'a', b'b'], vec![]],
    };
    let _ = set.longest_common_suffix();
}

