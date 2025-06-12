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

#[derive(Debug)]
struct Char(char);

impl From<char> for Char {
    fn from(c: char) -> Self {
        Char(c)
    }
}

fn decode_last_utf8(input: &str) -> Option<(char, usize)> {
    // A simple mock implementation for the purpose of this test
    if input.is_empty() {
        None
    } else {
        let last_char = input.chars().last().unwrap();
        Some((last_char, last_char.len_utf8()))
    }
}

struct Input<'a>(&'a str);

impl<'a> Input<'a> {
    fn previous_char(&self, at: InputAt) -> Char {
        decode_last_utf8(&self.0[..at.pos()]).map(|(c, _)| c).into()
    }
}

#[test]
fn test_previous_char_valid() {
    let input = Input("hello");
    let at = InputAt { position: 5 }; // Position at the end of "hello"
    let result = input.previous_char(at);
    assert_eq!(result.0, 'o');
}

#[test]
fn test_previous_char_boundary() {
    let input = Input("hello");
    let at = InputAt { position: 0 }; // Position at the beginning
    let result = input.previous_char(at);
    assert_eq!(result.0, 'h');
}

#[test]
fn test_previous_char_empty() {
    let input = Input("");
    let at = InputAt { position: 0 }; // Position at the beginning of an empty string
    let result = input.previous_char(at);
    // Assuming we expect some default behavior for empty, we need to define what this would be in real implementation
    assert_eq!(result.0, ' '); // Placeholder behavior for empty input
}

