// Answer 0

#[derive(Debug)]
struct Parser {
    chars: Vec<char>,
    position: usize,
}

impl Parser {
    fn new(chars: Vec<char>) -> Self {
        Parser { chars, position: 0 }
    }

    fn char(&self) -> char {
        if self.position < self.chars.len() {
            self.chars[self.position]
        } else {
            panic!("current position does not point to a valid char")
        }
    }

    fn offset(&self) -> usize {
        self.position
    }

    fn advance(&mut self) {
        if self.position < self.chars.len() {
            self.position += 1;
        }
    }
}

#[test]
fn test_char_valid_position() {
    let parser = Parser::new(vec!['a', 'b', 'c']);
    assert_eq!(parser.char(), 'a');
}

#[test]
#[should_panic(expected = "current position does not point to a valid char")]
fn test_char_invalid_position() {
    let mut parser = Parser::new(vec!['a', 'b', 'c']);
    parser.advance();
    parser.advance();
    parser.advance(); // now at an invalid position
    parser.char(); // this should panic
}

#[test]
fn test_char_boundary_conditions() {
    let parser = Parser::new(vec!['x']);
    assert_eq!(parser.char(), 'x'); // valid at start
}

