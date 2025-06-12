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
    is_word: bool,
}

impl StateFlags {
    fn set_word(&mut self) {
        self.is_word = true;
    }
}

#[derive(Default)]
struct Byte;

impl Byte {
    fn byte(b: u8) -> Self {
        Byte
    }

    fn is_ascii_word(self) -> bool {
        true // Mocking the behavior for test case
    }
}

struct DFA;

impl DFA {
    fn start_flags(&self, text: &[u8], at: usize) -> (EmptyFlags, StateFlags) {
        let mut empty_flags = EmptyFlags::default();
        let mut state_flags = StateFlags::default();
        empty_flags.start = at == 0;
        empty_flags.end = text.is_empty();
        empty_flags.start_line = at == 0 || text[at - 1] == b'\n';
        empty_flags.end_line = text.is_empty();

        let is_word_last = at > 0 && Byte::byte(text[at - 1]).is_ascii_word();
        let is_word = at < text.len() && Byte::byte(text[at]).is_ascii_word();
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
fn test_start_flags_not_word_boundary() {
    let dfa = DFA;
    let text = b"hello world";
    let at = 5; // "hello " ends with a word
    let (empty_flags, state_flags) = dfa.start_flags(text, at);

    assert_eq!(empty_flags.start, false);
    assert_eq!(empty_flags.end, false);
    assert_eq!(empty_flags.start_line, false);
    assert_eq!(empty_flags.end_line, false);
    assert_eq!(empty_flags.not_word_boundary, true);
    assert_eq!(empty_flags.word_boundary, false);
    assert_eq!(state_flags.is_word, true);
}

#[test]
fn test_start_flags_word_boundary() {
    let dfa = DFA;
    let text = b"hello ";
    let at = 6; // At end of the word "hello"
    let (empty_flags, state_flags) = dfa.start_flags(text, at);

    assert_eq!(empty_flags.start, false);
    assert_eq!(empty_flags.end, false);
    assert_eq!(empty_flags.start_line, false);
    assert_eq!(empty_flags.end_line, false);
    assert_eq!(empty_flags.not_word_boundary, false);
    assert_eq!(empty_flags.word_boundary, true);
    assert_eq!(state_flags.is_word, false);
}

