// Answer 0

#[derive(Debug)]
struct Literal {
    len: usize,
}

impl Literal {
    fn len(&self) -> usize {
        self.len
    }
}

#[derive(Debug)]
struct LiteralSet {
    lits: Vec<Literal>,
}

impl LiteralSet {
    fn new(lits: Vec<Literal>) -> Self {
        LiteralSet { lits }
    }

    pub fn min_len(&self) -> Option<usize> {
        let mut min = None;
        for lit in &self.lits {
            match min {
                None => min = Some(lit.len()),
                Some(m) if lit.len() < m => min = Some(lit.len()),
                _ => {}
            }
        }
        min
    }
}

#[test]
fn test_min_len_none() {
    let set = LiteralSet::new(vec![]);
    assert_eq!(set.min_len(), None);
}

#[test]
fn test_min_len_single_literal() {
    let set = LiteralSet::new(vec![Literal { len: 4 }]);
    assert_eq!(set.min_len(), Some(4));
}

#[test]
fn test_min_len_multiple_literals_different_lengths() {
    let set = LiteralSet::new(vec![Literal { len: 6 }, Literal { len: 3 }, Literal { len: 5 }]);
    assert_eq!(set.min_len(), Some(3));
}

#[test]
fn test_min_len_multiple_literals_same_length() {
    let set = LiteralSet::new(vec![Literal { len: 5 }, Literal { len: 5 }, Literal { len: 5 }]);
    assert_eq!(set.min_len(), Some(5));
}

#[test]
fn test_min_len_multiple_literals_with_some_longer() {
    let set = LiteralSet::new(vec![Literal { len: 7 }, Literal { len: 2 }, Literal { len: 7 }]);
    assert_eq!(set.min_len(), Some(2));
}

