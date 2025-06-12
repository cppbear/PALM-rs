// Answer 0

#[derive(Debug)]
struct Parser {
    input: Vec<char>,
    position: usize,
    ignore_whitespace: bool,
}

impl Parser {
    fn new(input: &str, ignore_whitespace: bool) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            ignore_whitespace,
        }
    }

    fn ignore_whitespace(&self) -> bool {
        self.ignore_whitespace
    }

    fn is_eof(&self) -> bool {
        self.position >= self.input.len()
    }

    fn char(&self) -> char {
        self.input[self.position]
    }

    fn bump(&mut self) {
        if !self.is_eof() {
            self.position += 1;
        }
    }

    fn pos(&self) -> usize {
        self.position
    }

    fn bump_space(&mut self) {
        if !self.ignore_whitespace() {
            return;
        }
        while !self.is_eof() {
            if self.char().is_whitespace() {
                self.bump();
            } else if self.char() == '#' {
                let start = self.pos();
                let mut comment_text = String::new();
                self.bump();
                while !self.is_eof() {
                    let c = self.char();
                    self.bump();
                    if c == '\n' {
                        break;
                    }
                    comment_text.push(c);
                }
                // Assuming there's a parser.comments to collect comments
            } else {
                break;
            }
        }
    }
}

#[test]
fn test_bump_space_no_whitespace_no_comment() {
    let mut parser = Parser::new("abc", true);
    parser.bump_space();
    assert_eq!(parser.pos(), 0);  // No change, since there's no whitespace or comment
}

#[test]
fn test_bump_space_with_trailing_whitespace() {
    let mut parser = Parser::new("abc   ", true);
    parser.bump_space();
    assert_eq!(parser.pos(), 0);  // Moves to end since ignores whitespace
}

#[test]
fn test_bump_space_with_comment() {
    let mut parser = Parser::new("abc # this is a comment\n def", true);
    parser.bump_space();
    assert_eq!(parser.pos(), 0);  // No change since bump_space only advances through whitespace/comments
}

#[test]
fn test_bump_space_reaches_eof() {
    let mut parser = Parser::new("  # comment line 1\n  # comment line 2\n   ", true);
    parser.bump_space();
    assert!(parser.is_eof());  // Resulting position should be at the end
}

