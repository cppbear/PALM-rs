// Answer 0

#[derive(Debug)]
struct NFA {
    captures: Vec<usize>,
}

struct Regex {
    nfa: NFA,
}

impl Regex {
    fn slots_len(&self) -> usize {
        self.nfa.captures.len() * 2
    }
}

#[test]
fn test_slots_len_empty_captures() {
    let regex = Regex { nfa: NFA { captures: Vec::new() } };
    assert_eq!(regex.slots_len(), 0);
}

#[test]
fn test_slots_len_one_capture() {
    let regex = Regex { nfa: NFA { captures: vec![1] } };
    assert_eq!(regex.slots_len(), 2);
}

#[test]
fn test_slots_len_multiple_captures() {
    let regex = Regex { nfa: NFA { captures: vec![1, 2, 3] } };
    assert_eq!(regex.slots_len(), 6);
}

