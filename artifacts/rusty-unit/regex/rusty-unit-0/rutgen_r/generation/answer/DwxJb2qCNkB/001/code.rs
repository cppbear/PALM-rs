// Answer 0

struct Hir {
    end: char,
}

impl Hir {
    fn set_upper(&mut self, bound: char) {
        self.end = bound;
    }
}

#[test]
fn test_set_upper() {
    let mut hir = Hir { end: 'a' };

    // Test normal case
    hir.set_upper('z');
    assert_eq!(hir.end, 'z');

    // Test boundary case
    hir.set_upper('A');
    assert_eq!(hir.end, 'A');

    // Test setting to the same character
    hir.set_upper('A');
    assert_eq!(hir.end, 'A');

    // Test setting to a lower boundary character
    hir.set_upper(' ');
    assert_eq!(hir.end, ' ');

    // Test a panic case (not directly applicable as it doesn't panic under normal circumstances)
    // If there were a valid panic case we would use #[should_panic]
}

