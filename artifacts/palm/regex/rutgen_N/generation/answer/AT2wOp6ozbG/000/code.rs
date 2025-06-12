// Answer 0

#[derive(Default)]
struct CharClass {
    set: Vec<char>,
}

impl CharClass {
    pub fn negate(&mut self) {
        // Assuming we have a simple implementation for negating the class
        let all_chars: Vec<char> = ('\u{0000}'..='\u{FFFF}').collect();
        self.set = all_chars.into_iter().filter(|c| !self.set.contains(c)).collect();
    }
}

#[test]
fn test_negate_empty_set() {
    let mut char_class = CharClass::default();
    char_class.set = Vec::new();
    char_class.negate();
    assert_eq!(char_class.set.len(), 65536); // All unicode characters
}

#[test]
fn test_negate_full_set() {
    let mut char_class = CharClass::default();
    char_class.set = ('\u{0000}'..='\u{FFFF}').collect();
    char_class.negate();
    assert!(char_class.set.is_empty());
}

#[test]
fn test_negate_partial_set() {
    let mut char_class = CharClass::default();
    char_class.set = vec!['a', 'b', 'c'];
    char_class.negate();
    assert!(char_class.set.iter().all(|&c| c != 'a' && c != 'b' && c != 'c'));
    assert_eq!(char_class.set.len(), 65536 - 3); // All other characters except 'a', 'b', 'c'
}

