// Answer 0

#[derive(Debug)]
struct HIR {
    start: u8,
}

impl HIR {
    fn new() -> Self {
        HIR { start: 0 }
    }

    fn set_lower(&mut self, bound: u8) {
        self.start = bound;
    }
}

#[test]
fn test_set_lower() {
    let mut hir = HIR::new();
    hir.set_lower(5);
    assert_eq!(hir.start, 5);
}

#[test]
fn test_set_lower_to_zero() {
    let mut hir = HIR::new();
    hir.set_lower(0);
    assert_eq!(hir.start, 0);
}

#[test]
fn test_set_lower_overflow() {
    let mut hir = HIR::new();
    hir.set_lower(255);
    assert_eq!(hir.start, 255);
}

