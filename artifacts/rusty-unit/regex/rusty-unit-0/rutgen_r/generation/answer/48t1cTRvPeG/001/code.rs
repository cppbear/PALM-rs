// Answer 0

#[derive(Debug)]
struct Dfa(u8);

impl Dfa {
    fn has_empty(&self) -> bool {
        self.0 & 0b00000_1_00 > 0
    }
}

#[test]
fn test_has_empty_true() {
    let dfa = Dfa(0b00000_1_00); // This should return true
    assert_eq!(dfa.has_empty(), true);
}

#[test]
fn test_has_empty_false() {
    let dfa = Dfa(0b00000_0_00); // This should return false
    assert_eq!(dfa.has_empty(), false);
}

#[test]
fn test_has_empty_edge_case() {
    let dfa = Dfa(0b00000_1_01); // This should still return true
    assert_eq!(dfa.has_empty(), true);
}

#[test]
fn test_has_empty_all_ones() {
    let dfa = Dfa(0b11111_1_11); // This should return true
    assert_eq!(dfa.has_empty(), true);
}

#[test]
fn test_has_empty_all_zeros() {
    let dfa = Dfa(0b00000_0_00); // This should return false
    assert_eq!(dfa.has_empty(), false);
}

