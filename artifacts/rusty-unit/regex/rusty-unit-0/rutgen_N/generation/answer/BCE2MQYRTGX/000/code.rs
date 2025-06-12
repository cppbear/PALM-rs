// Answer 0

#[derive(Debug)]
struct WordSet(u8);

impl WordSet {
    fn set_word(&mut self) {
        self.0 |= 0b000000_1_0;
    }
}

#[test]
fn test_set_word() {
    let mut word_set = WordSet(0b000000_0_0);
    word_set.set_word();
    assert_eq!(word_set.0, 0b000000_1_0);
}

#[test]
fn test_set_word_already_set() {
    let mut word_set = WordSet(0b000000_1_0);
    word_set.set_word();
    assert_eq!(word_set.0, 0b000000_1_0);
}

#[test]
fn test_set_word_with_other_bits() {
    let mut word_set = WordSet(0b000001_1_0);
    word_set.set_word();
    assert_eq!(word_set.0, 0b000001_1_0);
}

