// Answer 0

#[derive(Debug)]
struct Char {
    value: Option<char>,
}

#[derive(Debug)]
struct Input {
    c: Char,
}

impl Input {
    pub fn new(c: Option<char>) -> Self {
        Input { c: Char { value: c } }
    }

    pub fn char(&self) -> Char {
        self.c.clone()
    }
}

#[test]
fn test_char_return_value() {
    let input = Input::new(Some('a'));
    let result = input.char();
    assert_eq!(result.value, Some('a'));
}

#[test]
fn test_char_return_none() {
    let input = Input::new(None);
    let result = input.char();
    assert_eq!(result.value, None);
}

