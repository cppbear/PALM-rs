// Answer 0


struct InputAt {
    input: String,
    position: usize,
}

impl InputAt {
    fn new(input: String, position: usize) -> Self {
        Self { input, position }
    }

    fn char(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }
}

struct Char(Option<char>);

impl Char {
    fn new(character: Option<char>) -> Self {
        Self(character)
    }
}

fn next_char(at: InputAt) -> Char {
    Char::new(at.char())
}

#[test]
fn test_next_char_valid_position() {
    let input = InputAt::new("hello".to_string(), 1);
    let result = next_char(input);
    assert_eq!(result.0, Some('e'));
}

#[test]
fn test_next_char_boundary_condition_start() {
    let input = InputAt::new("world".to_string(), 0);
    let result = next_char(input);
    assert_eq!(result.0, Some('w'));
}

#[test]
fn test_next_char_boundary_condition_end() {
    let input = InputAt::new("test".to_string(), 3);
    let result = next_char(input);
    assert_eq!(result.0, Some('t'));
}

#[test]
fn test_next_char_invalid_position() {
    let input = InputAt::new("valid".to_string(), 10);
    let result = next_char(input);
    assert_eq!(result.0, None);
}

#[test]
fn test_next_char_empty_string() {
    let input = InputAt::new("".to_string(), 0);
    let result = next_char(input);
    assert_eq!(result.0, None);
}


