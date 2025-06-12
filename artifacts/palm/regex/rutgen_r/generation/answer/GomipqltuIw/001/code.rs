// Answer 0

#[derive(Debug)]
struct Literal {
    value: String,
}

impl Literal {
    fn reverse(&mut self) {
        self.value = self.value.chars().rev().collect();
    }
}

#[derive(Debug)]
struct Literals {
    lits: Vec<Literal>,
}

impl Literals {
    fn new(lits: Vec<Literal>) -> Self {
        Literals { lits }
    }

    pub fn reverse(&mut self) {
        for lit in &mut self.lits {
            lit.reverse();
        }
    }
}

#[test]
fn test_reverse_literals_non_empty() {
    let mut literals = Literals::new(vec![
        Literal { value: String::from("abc") },
        Literal { value: String::from("def") },
    ]);
    literals.reverse();
    assert_eq!(literals.lits[0].value, "cba");
    assert_eq!(literals.lits[1].value, "fed");
}

#[test]
fn test_reverse_literals_empty() {
    let mut literals = Literals::new(Vec::new());
    literals.reverse();
    assert!(literals.lits.is_empty());
}

#[test]
fn test_reverse_literals_single_item() {
    let mut literals = Literals::new(vec![
        Literal { value: String::from("xyz") },
    ]);
    literals.reverse();
    assert_eq!(literals.lits[0].value, "zyx");
}

#[should_panic]
#[test]
fn test_reverse_literals_invalid() {
    let mut literals = Literals::new(vec![
        Literal { value: String::from("") },
    ]);
    literals.reverse();
    assert_eq!(literals.lits[0].value, "not expected to panic");
}

