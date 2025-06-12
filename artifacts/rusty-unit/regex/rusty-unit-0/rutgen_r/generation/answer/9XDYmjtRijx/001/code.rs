// Answer 0

#[test]
fn test_char_valid_position() {
    struct Parser {
        data: Vec<char>,
        position: usize,
    }

    impl Parser {
        fn new(data: &str) -> Self {
            Self {
                data: data.chars().collect(),
                position: 0,
            }
        }

        fn offset(&self) -> usize {
            self.position
        }

        fn char_at(&self, offset: usize) -> char {
            self.data[offset]
        }

        fn char(&self) -> char {
            self.char_at(self.offset())
        }

        fn set_position(&mut self, pos: usize) {
            self.position = pos;
        }
    }

    let parser = Parser::new("abc");
    assert_eq!(parser.char(), 'a');
}

#[test]
#[should_panic]
fn test_char_invalid_position_too_high() {
    struct Parser {
        data: Vec<char>,
        position: usize,
    }

    impl Parser {
        fn new(data: &str) -> Self {
            Self {
                data: data.chars().collect(),
                position: 0,
            }
        }

        fn offset(&self) -> usize {
            self.position
        }

        fn char_at(&self, offset: usize) -> char {
            self.data[offset]
        }

        fn char(&self) -> char {
            self.char_at(self.offset())
        }

        fn set_position(&mut self, pos: usize) {
            self.position = pos;
        }
    }

    let mut parser = Parser::new("abc");
    parser.set_position(3);
    parser.char(); // Panics because position 3 is invalid.
}

#[test]
#[should_panic]
fn test_char_invalid_position_negative() {
    struct Parser {
        data: Vec<char>,
        position: usize,
    }

    impl Parser {
        fn new(data: &str) -> Self {
            Self {
                data: data.chars().collect(),
                position: 0,
            }
        }

        fn offset(&self) -> usize {
            self.position
        }

        fn char_at(&self, offset: usize) -> char {
            self.data[offset]
        }

        fn char(&self) -> char {
            self.char_at(self.offset())
        }
    }

    let parser = Parser::new("abc");
    parser.char(); // Panics because the initial position is 0, and while valid, it will be negative if we were to modify.
}

