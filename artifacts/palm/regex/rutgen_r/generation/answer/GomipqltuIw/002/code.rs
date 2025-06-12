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

struct Literals {
    lits: Vec<Literal>,
}

impl Literals {
    fn new() -> Self {
        Self {
            lits: Vec::new(),
        }
    }

    fn reverse(&mut self) {
        for lit in &mut self.lits {
            lit.reverse();
        }
    }
}

#[test]
fn test_reverse_with_multiple_literals() {
    let mut literals = Literals::new();
    literals.lits.push(Literal::new("abc"));
    literals.lits.push(Literal::new("def"));
    literals.lits.push(Literal::new("ghi"));

    literals.reverse();

    assert_eq!(literals.lits[0].value, "cba");
    assert_eq!(literals.lits[1].value, "fed");
    assert_eq!(literals.lits[2].value, "ihg");
}

#[test]
fn test_reverse_with_empty_literal() {
    let mut literals = Literals::new();
    literals.lits.push(Literal::new(""));

    literals.reverse();

    assert_eq!(literals.lits[0].value, "");
}

#[test]
fn test_reverse_with_single_literal() {
    let mut literals = Literals::new();
    literals.lits.push(Literal::new("xyz"));

    literals.reverse();

    assert_eq!(literals.lits[0].value, "zyx");
}

#[test]
fn test_reverse_with_no_literals() {
    let mut literals = Literals::new();

    literals.reverse();

    assert!(literals.lits.is_empty());
}

