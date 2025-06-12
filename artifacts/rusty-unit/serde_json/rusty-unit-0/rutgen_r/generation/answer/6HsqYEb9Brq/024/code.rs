// Answer 0

#[derive(Default)]
struct MockParser {
    chars: Vec<u8>,
    pos: usize,
}

impl MockParser {
    fn next_char_or_null(&mut self) -> Result<u8, ()> {
        if self.pos < self.chars.len() {
            let ch = self.chars[self.pos];
            self.pos += 1;
            Ok(ch)
        } else {
            Ok(0) // simulate null
        }
    }

    fn peek_or_null(&self) -> Result<u8, ()> {
        if self.pos < self.chars.len() {
            Ok(self.chars[self.pos])
        } else {
            Ok(0) // simulate null
        }
    }

    fn eat_char(&mut self) {
        if self.pos < self.chars.len() {
            self.pos += 1; // consume the character
        }
    }

    fn ignore_decimal(&mut self) -> Result<(), ()> {
        // Placeholder implementation for ignoring decimal
        Ok(())
    }

    fn ignore_exponent(&mut self) -> Result<(), ()> {
        // Placeholder implementation for ignoring exponent
        Ok(())
    }

    fn error(&self, _: ()) -> () {
        // Just a placeholder for error handling
    }

    fn peek_error(&self, _: ()) -> () {
        // Just a placeholder for error handling
    }
}

impl MockParser {
    fn ignore_integer(&mut self) -> Result<(), ()> {
        match self.next_char_or_null() {
            Ok(b'0') => {
                // There can be only one leading '0'.
                if let Ok(ch) = self.peek_or_null() {
                    if (b'0'..=b'9').contains(&ch) {
                        return Err(());
                    }
                }
            }
            Ok(b'1'..=b'9') => {
                while let Ok(ch) = self.peek_or_null() {
                    if (b'0'..=b'9').contains(&ch) {
                        self.eat_char();
                    } else {
                        break;
                    }
                }
            }
            _ => {
                return Err(());
            }
        }

        match self.peek_or_null() {
            Ok(b'.') => self.ignore_decimal(),
            Ok(b'e') | Ok(b'E') => self.ignore_exponent(),
            _ => Ok(()),
        }
    }
}

#[test]
fn test_ignore_integer_valid_single_digit() {
    let mut parser = MockParser { chars: vec![b'1'], pos: 0 };
    assert_eq!(parser.ignore_integer(), Ok(()));
}

#[test]
fn test_ignore_integer_valid_multiple_digits() {
    let mut parser = MockParser { chars: vec![b'2', b'3', b'4'], pos: 0 };
    assert_eq!(parser.ignore_integer(), Ok(()));
}

#[test]
fn test_ignore_integer_leading_zero_invalid() {
    let mut parser = MockParser { chars: vec![b'0', b'5'], pos: 0 };
    assert_eq!(parser.ignore_integer(), Err(()));
}

#[test]
fn test_ignore_integer_invalid_character() {
    let mut parser = MockParser { chars: vec![b'#'], pos: 0 };
    assert_eq!(parser.ignore_integer(), Err(()));
}

#[test]
fn test_ignore_integer_valid_decimal() {
    let mut parser = MockParser { chars: vec![b'3', b'.'], pos: 0 };
    assert_eq!(parser.ignore_integer(), Ok(()));
}

#[test]
fn test_ignore_integer_valid_exponent() {
    let mut parser = MockParser { chars: vec![b'4', b'e', b'5'], pos: 0 };
    assert_eq!(parser.ignore_integer(), Ok(()));
}

