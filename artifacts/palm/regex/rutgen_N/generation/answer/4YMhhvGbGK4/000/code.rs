// Answer 0

#[derive(Debug)]
struct Literal {
    complete: bool,
    empty: bool,
}

impl Literal {
    fn is_empty(&self) -> bool {
        self.empty
    }
    
    fn complete(&self) -> bool {
        self.complete && !self.is_empty()
    }
}

#[test]
fn test_complete_true() {
    let literal = Literal { complete: true, empty: false };
    assert!(literal.complete());
}

#[test]
fn test_complete_false_due_to_empty() {
    let literal = Literal { complete: true, empty: true };
    assert!(!literal.complete());
}

#[test]
fn test_complete_false_due_to_not_complete() {
    let literal = Literal { complete: false, empty: false };
    assert!(!literal.complete());
}

#[test]
fn test_complete_false_due_to_both_conditions() {
    let literal = Literal { complete: false, empty: true };
    assert!(!literal.complete());
}

