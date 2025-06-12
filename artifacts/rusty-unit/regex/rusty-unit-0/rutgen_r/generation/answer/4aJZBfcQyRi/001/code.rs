// Answer 0

#[derive(Default)]
struct EmptyFlags {
    start: bool,
    end: bool,
    start_line: bool,
    end_line: bool,
    not_word_boundary: bool,
    word_boundary: bool,
}

#[derive(Default)]
struct StateFlags {
    word: bool,
}

impl StateFlags {
    fn set_word(&mut self) {
        self.word = true;
    }
}

struct Byte(u8);

impl Byte {
    fn byte(b: u8) -> Self {
        Byte(b)
    }

    fn is_ascii_word(&self) -> bool {
        self.0.is_ascii_alphanumeric() || self.0 == b'_'
    }
}

struct MyDFA;

impl MyDFA {
    fn start_flags_reverse(&self, text: &[u8], at: usize) -> (EmptyFlags, StateFlags) {
        let mut empty_flags = EmptyFlags::default();
        let mut state_flags = StateFlags::default();
        empty_flags.start = at == text.len();
        empty_flags.end = text.is_empty();
        empty_flags.start_line = at == text.len() || text[at] == b'\n';
        empty_flags.end_line = text.is_empty();

        let is_word_last = at < text.len() && Byte::byte(text[at]).is_ascii_word();
        let is_word = at > 0 && Byte::byte(text[at - 1]).is_ascii_word();
        if is_word_last {
            state_flags.set_word();
        }
        if is_word == is_word_last {
            empty_flags.not_word_boundary = true;
        } else {
            empty_flags.word_boundary = true;
        }
        (empty_flags, state_flags)
    }
}

#[test]
fn test_start_flags_reverse_at_text_len() {
    let dfa = MyDFA;
    let text = b"test";
    let at = text.len();
    let (empty_flags, state_flags) = dfa.start_flags_reverse(text, at);
    
    assert_eq!(empty_flags.start, true);
    assert_eq!(empty_flags.end, false);
    assert_eq!(empty_flags.start_line, true);
    assert_eq!(empty_flags.end_line, false);
    assert_eq!(empty_flags.not_word_boundary, false);
    assert_eq!(empty_flags.word_boundary, false);
    assert_eq!(state_flags.word, false);
}

#[test]
fn test_start_flags_reverse_at_less_than_text_len() {
    let dfa = MyDFA;
    let text = b"test";
    let at = 2; // is_word_last should be true since 's' is a word character
    let (empty_flags, state_flags) = dfa.start_flags_reverse(text, at);
    
    assert_eq!(empty_flags.start, false);
    assert_eq!(empty_flags.end, false);
    assert_eq!(empty_flags.start_line, false);
    assert_eq!(empty_flags.end_line, false);
    assert_eq!(empty_flags.not_word_boundary, true);
    assert_eq!(empty_flags.word_boundary, false);
    assert_eq!(state_flags.word, true);
}

#[test]
fn test_start_flags_reverse_at_greater_than_zero() {
    let dfa = MyDFA;
    let text = b"word";
    let at = 1; // is_word_last should be false since 'o' is a word character, but it's not the last one
    let (empty_flags, state_flags) = dfa.start_flags_reverse(text, at);
    
    assert_eq!(empty_flags.start, false);
    assert_eq!(empty_flags.end, false);
    assert_eq!(empty_flags.start_line, false);
    assert_eq!(empty_flags.end_line, false);
    assert_eq!(empty_flags.not_word_boundary, false);
    assert_eq!(empty_flags.word_boundary, true);
    assert_eq!(state_flags.word, false);
}

