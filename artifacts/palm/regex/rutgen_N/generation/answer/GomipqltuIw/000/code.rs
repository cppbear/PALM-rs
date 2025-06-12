// Answer 0

#[derive(Debug)]
struct Literal {
    value: String,
}

impl Literal {
    fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }

    fn reverse(&mut self) {
        self.value = self.value.chars().rev().collect();
    }
}

#[derive(Debug)]
struct LiteralCollection {
    lits: Vec<Literal>,
}

impl LiteralCollection {
    fn new(lits: Vec<Literal>) -> Self {
        Self { lits }
    }

    pub fn reverse(&mut self) {
        for lit in &mut self.lits {
            lit.reverse();
        }
    }
}

#[test]
fn test_reverse_literals() {
    let mut collection = LiteralCollection::new(vec![
        Literal::new("one"),
        Literal::new("two"),
        Literal::new("three"),
    ]);

    collection.reverse();

    assert_eq!(collection.lits[0].value, "eno");
    assert_eq!(collection.lits[1].value, "owt");
    assert_eq!(collection.lits[2].value, "eerht");
}

#[test]
fn test_reverse_empty_collection() {
    let mut collection = LiteralCollection::new(vec![]);
    collection.reverse();
    assert!(collection.lits.is_empty());
}

#[test]
fn test_reverse_single_literal() {
    let mut collection = LiteralCollection::new(vec![
        Literal::new("single"),
    ]);
    
    collection.reverse();
    
    assert_eq!(collection.lits[0].value, "elgnis");
}

