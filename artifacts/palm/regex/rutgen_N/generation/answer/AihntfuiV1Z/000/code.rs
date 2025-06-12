// Answer 0

#[derive(Debug)]
struct Literal {
    lits: Vec<String>,
}

impl Literal {
    fn new(lits: Vec<String>) -> Self {
        Literal { lits }
    }

    fn num_bytes(&self) -> usize {
        self.lits.iter().fold(0, |accum, lit| accum + lit.len())
    }
}

#[test]
fn test_num_bytes_empty() {
    let literal = Literal::new(Vec::new());
    assert_eq!(literal.num_bytes(), 0);
}

#[test]
fn test_num_bytes_single_literal() {
    let literal = Literal::new(vec![String::from("a")]);
    assert_eq!(literal.num_bytes(), 1);
}

#[test]
fn test_num_bytes_multiple_literals() {
    let literal = Literal::new(vec![String::from("abc"), String::from("def")]);
    assert_eq!(literal.num_bytes(), 6);
}

#[test]
fn test_num_bytes_with_empty_literal() {
    let literal = Literal::new(vec![String::from(""), String::from("test")]);
    assert_eq!(literal.num_bytes(), 4);
}

#[test]
fn test_num_bytes_non_ascii() {
    let literal = Literal::new(vec![String::from("你好"), String::from("world")]);
    assert_eq!(literal.num_bytes(), 10); // 6 bytes for "你好", 5 bytes for "world"
}

