// Answer 0

struct TestParser {
    pattern: String,
    pos: usize,
    eof: bool,
}

impl TestParser {
    fn new(pattern: &str) -> Self {
        Self {
            pattern: pattern.to_string(),
            pos: 0,
            eof: false,
        }
    }

    fn is_eof(&self) -> bool {
        self.eof
    }

    fn char(&self) -> char {
        if self.is_eof() {
            return '\0';
        }
        self.pattern.chars().nth(self.pos).unwrap_or('\0')
    }

    fn bump(&mut self) -> bool {
        if self.is_eof() {
            return false;
        }
        self.pos += 1;
        if self.pos >= self.pattern.len() {
            self.eof = true;
        }
        true
    }

    fn span(&self) -> usize {
        self.pos
    }

    fn span_char(&self) -> usize {
        self.pos
    }

    fn pattern(&self) -> &str {
        &self.pattern
    }

    fn add_capture_name(&self, _capname: &ast::CaptureName) -> Result<(), ()> {
        Ok(())
    }

    fn error(&self, _span: usize, _kind: ast::ErrorKind) -> Result<ast::CaptureName, ()> {
        Err(())
    }
}

mod ast {
    #[derive(Debug, PartialEq)]
    pub struct CaptureName {
        pub span: usize,
        pub name: String,
        pub index: u32,
    }

    #[derive(Debug)]
    pub enum ErrorKind {
        GroupNameUnexpectedEof,
        GroupNameInvalid,
        GroupNameEmpty,
    }
}

#[test]
fn test_parse_capture_name_valid() {
    let mut parser = TestParser::new("<validName>");
    let result = parser.parse_capture_name(1);
    assert!(result.is_ok());
    let capname = result.unwrap();
    assert_eq!(capname.name, "validName");
    assert_eq!(capname.index, 1);
}

#[test]
fn test_parse_capture_name_empty() {
    let mut parser = TestParser::new("<>");
    let result = parser.parse_capture_name(1);
    assert!(result.is_err());
}

#[test]
fn test_parse_capture_name_invalid_chars() {
    let mut parser = TestParser::new("<invalid@name>");
    let result = parser.parse_capture_name(1);
    assert!(result.is_err());
}

#[test]
fn test_parse_capture_name_unexpected_eof() {
    let mut parser = TestParser::new("<validName");
    parser.eof = true; // Simulate EOF
    let result = parser.parse_capture_name(1);
    assert!(result.is_err());
}

#[test]
fn test_parse_capture_name_success_after_bump_fail() {
    let mut parser = TestParser::new("<someName>");
    parser.bump(); // simulate first bump that succeeds
    let result = parser.parse_capture_name(2);
    assert!(result.is_ok());
}

