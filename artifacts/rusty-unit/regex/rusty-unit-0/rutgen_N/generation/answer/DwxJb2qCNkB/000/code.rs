// Answer 0

#[derive(Default)]
struct HIR {
    end: char,
}

impl HIR {
    fn set_upper(&mut self, bound: char) {
        self.end = bound;
    }
}

#[test]
fn test_set_upper_updates_end_value() {
    let mut hir = HIR::default();
    hir.set_upper('Z');
    assert_eq!(hir.end, 'Z');
}

#[test]
fn test_set_upper_with_lowercase() {
    let mut hir = HIR::default();
    hir.set_upper('a');
    assert_eq!(hir.end, 'a');
}

#[test]
fn test_set_upper_with_special_character() {
    let mut hir = HIR::default();
    hir.set_upper('!');
    assert_eq!(hir.end, '!');
}

#[test]
fn test_set_upper_with_numeric_character() {
    let mut hir = HIR::default();
    hir.set_upper('9');
    assert_eq!(hir.end, '9');
}

