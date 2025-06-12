// Answer 0

#[derive(Debug)]
struct StateFlags {
    match_flag: bool,
    word_flag: bool,
    empty_flag: bool,
}

impl StateFlags {
    fn is_match(&self) -> bool {
        self.match_flag
    }
    
    fn is_word(&self) -> bool {
        self.word_flag
    }
    
    fn has_empty(&self) -> bool {
        self.empty_flag
    }
}

use std::fmt;

impl fmt::Debug for StateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("StateFlags")
            .field("is_match", &self.is_match())
            .field("is_word", &self.is_word())
            .field("has_empty", &self.has_empty())
            .finish()
    }
}

#[test]
fn test_state_flags_all_true() {
    let state = StateFlags {
        match_flag: true,
        word_flag: true,
        empty_flag: true,
    };
    let expected = "StateFlags { is_match: true, is_word: true, has_empty: true }";
    assert!(format!("{:?}", state) == expected);
}

#[test]
fn test_state_flags_all_false() {
    let state = StateFlags {
        match_flag: false,
        word_flag: false,
        empty_flag: false,
    };
    let expected = "StateFlags { is_match: false, is_word: false, has_empty: false }";
    assert!(format!("{:?}", state) == expected);
}

#[test]
fn test_state_flags_mixed_values() {
    let state = StateFlags {
        match_flag: true,
        word_flag: false,
        empty_flag: true,
    };
    let expected = "StateFlags { is_match: true, is_word: false, has_empty: true }";
    assert!(format!("{:?}", state) == expected);
}

