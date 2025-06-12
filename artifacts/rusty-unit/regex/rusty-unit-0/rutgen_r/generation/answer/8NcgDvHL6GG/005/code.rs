// Answer 0

#[derive(Debug)]
struct LiteralSet {
    lits: Vec<Literal>,
}

#[derive(Debug)]
struct Literal {
    len: usize,
}

impl Literal {
    fn new(len: usize) -> Self {
        Literal { len }
    }

    fn len(&self) -> usize {
        self.len
    }
}

impl LiteralSet {
    fn min_len(&self) -> Option<usize> {
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
fn test_min_len_with_no_literals() {
    let set = LiteralSet { lits: vec![] };
    assert_eq!(set.min_len(), None);
}

#[test]
fn test_min_len_with_one_literal() {
    let set = LiteralSet { lits: vec![Literal::new(5)] };
    assert_eq!(set.min_len(), Some(5));
}

#[test]
fn test_min_len_with_multiple_literals() {
    let set = LiteralSet { lits: vec![Literal::new(8), Literal::new(3), Literal::new(6)] };
    assert_eq!(set.min_len(), Some(3));
}

#[test]
fn test_min_len_with_identical_literals() {
    let set = LiteralSet { lits: vec![Literal::new(4), Literal::new(4), Literal::new(4)] };
    assert_eq!(set.min_len(), Some(4));
}

#[test]
fn test_min_len_with_all_increasing_literals() {
    let set = LiteralSet { lits: vec![Literal::new(2), Literal::new(5), Literal::new(10)] };
    assert_eq!(set.min_len(), Some(2));
}

#[test]
fn test_min_len_with_all_decreasing_literals() {
    let set = LiteralSet { lits: vec![Literal::new(10), Literal::new(5), Literal::new(2)] };
    assert_eq!(set.min_len(), Some(2));
}

