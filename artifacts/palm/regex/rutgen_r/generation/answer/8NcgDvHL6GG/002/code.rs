// Answer 0

#[derive(Debug)]
struct Literal {
    content: String,
}

impl Literal {
    fn len(&self) -> usize {
        self.content.len()
    }
}

#[derive(Debug)]
struct LiteralSet {
    lits: Vec<Literal>,
}

impl LiteralSet {
    fn new(lits: Vec<Literal>) -> Self {
        Self { lits }
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
fn test_min_len_no_literals() {
    let set = LiteralSet::new(vec![]);
    assert_eq!(set.min_len(), None);
}

#[test]
fn test_min_len_one_literal() {
    let set = LiteralSet::new(vec![Literal { content: String::from("a") }]);
    assert_eq!(set.min_len(), Some(1));
}

#[test]
fn test_min_len_multiple_literals() {
    let set = LiteralSet::new(vec![
        Literal { content: String::from("abc") },
        Literal { content: String::from("a") },
        Literal { content: String::from("ab") },
    ]);
    assert_eq!(set.min_len(), Some(1));
}

#[test]
fn test_min_len_all_same_length_literals() {
    let set = LiteralSet::new(vec![
        Literal { content: String::from("xy") },
        Literal { content: String::from("zy") },
    ]);
    assert_eq!(set.min_len(), Some(2));
}

#[test]
fn test_min_len_complex_case() {
    let set = LiteralSet::new(vec![
        Literal { content: String::from("longer") },
        Literal { content: String::from("long") },
        Literal { content: String::from("tiny") },
        Literal { content: String::from("a") },
    ]);
    assert_eq!(set.min_len(), Some(1));
}

