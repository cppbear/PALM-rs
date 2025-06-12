// Answer 0

#[derive(Debug)]
struct InputAt {
    position: usize,
}

impl InputAt {
    fn pos(&self) -> usize {
        self.position
    }
}

struct Input {
    data: String,
}

impl Input {
    fn new(data: &str) -> Self {
        Input { data: data.to_string() }
    }

    fn next_char(&self, at: InputAt) -> char {
        let bytes = &self.data[at.pos()..];
        match std::str::from_utf8(bytes) {
            Ok(utf8_str) => utf8_str.chars().next().unwrap(),
            Err(_) => '\u{FFFD}', // Replacement character for invalid UTF-8
        }
    }
}

#[test]
fn test_next_char_valid_utf8() {
    let input = Input::new("hello");
    let at = InputAt { position: 0 };
    assert_eq!(input.next_char(at), 'h');
}

#[test]
fn test_next_char_middle() {
    let input = Input::new("hello");
    let at = InputAt { position: 1 };
    assert_eq!(input.next_char(at), 'e');
}

#[test]
fn test_next_char_end() {
    let input = Input::new("hello");
    let at = InputAt { position: 4 };
    assert_eq!(input.next_char(at), 'o');
}

#[test]
fn test_next_char_out_of_bounds() {
    let input = Input::new("hello");
    let at = InputAt { position: 5 };
    assert_eq!(input.next_char(at), '\u{FFFD}');
}

