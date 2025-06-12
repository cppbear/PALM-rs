// Answer 0

#[derive(Default)]
struct Parser {
    position: usize,
    input: String,
}

impl Parser {
    fn bump(&mut self) -> bool {
        if self.position < self.input.len() {
            self.position += 1;
            true
        } else {
            false
        }
    }

    fn bump_space(&mut self) {
        while self.position < self.input.len() && self.input[self.position..].starts_with(' ') {
            self.position += 1;
        }
    }

    fn is_eof(&self) -> bool {
        self.position >= self.input.len()
    }

    fn bump_and_bump_space(&mut self) -> bool {
        if !self.bump() {
            return false;
        }
        self.bump_space();
        !self.is_eof()
    }
}

#[test]
fn test_bump_and_bump_space_with_text() {
    let mut parser = Parser {
        position: 0,
        input: String::from("abc def"),
    };
    assert_eq!(parser.bump_and_bump_space(), true);
    assert_eq!(parser.position, 3); // position should be at 'd'
}

#[test]
fn test_bump_and_bump_space_with_leading_spaces() {
    let mut parser = Parser {
        position: 0,
        input: String::from("   abc def"),
    };
    assert_eq!(parser.bump_and_bump_space(), true);
    assert_eq!(parser.position, 3); // position should be at 'a'
}

#[test]
fn test_bump_and_bump_space_with_trailing_spaces() {
    let mut parser = Parser {
        position: 0,
        input: String::from("abc def   "),
    };
    assert_eq!(parser.bump_and_bump_space(), true);
    assert_eq!(parser.position, 3); // position should be at 'd'
}

#[test]
fn test_bump_and_bump_space_at_eof() {
    let mut parser = Parser {
        position: 0,
        input: String::from(""),
    };
    assert_eq!(parser.bump_and_bump_space(), false); // no input to bump
}

#[test]
fn test_bump_and_bump_space_after_eof() {
    let mut parser = Parser {
        position: 1,
        input: String::from(""),
    };
    assert_eq!(parser.bump_and_bump_space(), false); // already at eof
}

