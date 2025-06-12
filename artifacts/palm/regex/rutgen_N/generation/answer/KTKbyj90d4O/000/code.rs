// Answer 0

#[derive(Debug)]
struct Parser {
    pattern: String,
    pos: usize,
}

impl Parser {
    fn new(pattern: String) -> Self {
        Self { pattern, pos: 0 }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.pattern.len()
    }

    fn char(&self) -> char {
        self.pattern.chars().nth(self.pos).unwrap_or('\0')
    }

    fn bump(&mut self) -> bool {
        if self.is_eof() {
            false
        } else {
            self.pos += 1;
            true
        }
    }

    fn pos(&self) -> usize {
        self.pos
    }

    fn error(&self, span: Span, kind: ast::ErrorKind) -> String {
        format!("Error at {}: {:?}", span, kind)
    }

    fn add_capture_name(&self, _capname: &ast::CaptureName) -> Result<(), String> {
        Ok(())
    }

    fn pattern(&self) -> &String {
        &self.pattern
    }
}

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

impl Span {
    fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

mod ast {
    #[derive(Debug)]
    pub struct CaptureName {
        pub span: super::Span,
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

fn is_capture_char(c: char, is_first: bool) -> bool {
    c.is_alphanumeric() || (is_first && c == '_')
}

impl Parser {
    fn parse_capture_name(&mut self, capture_index: u32) -> Result<ast::CaptureName, String> {
        if self.is_eof() {
            return Err(self.error(self.pos(), ast::ErrorKind::GroupNameUnexpectedEof));
        }
        let start = self.pos();
        loop {
            if self.char() == '>' {
                break;
            }
            if !is_capture_char(self.char(), self.pos() == start) {
                return Err(self.error(self.pos(), ast::ErrorKind::GroupNameInvalid));
            }
            if !self.bump() {
                break;
            }
        }
        let end = self.pos();
        if self.is_eof() {
            return Err(self.error(self.pos(), ast::ErrorKind::GroupNameUnexpectedEof));
        }
        assert_eq!(self.char(), '>');
        self.bump();
        let name = &self.pattern()[start..end];
        if name.is_empty() {
            return Err(self.error(Span::new(start, start), ast::ErrorKind::GroupNameEmpty));
        }
        let capname = ast::CaptureName {
            span: Span::new(start, end),
            name: name.to_string(),
            index: capture_index,
        };
        self.add_capture_name(&capname)?;
        Ok(capname)
    }
}

#[test]
fn test_parse_capture_name_valid() {
    let mut parser = Parser::new("<group>".to_string());
    let result = parser.parse_capture_name(1);
    assert!(result.is_ok());
    let capname = result.unwrap();
    assert_eq!(capname.name, "group");
    assert_eq!(capname.index, 1);
}

#[test]
#[should_panic(expected = "Error at 7: GroupNameUnexpectedEof")]
fn test_parse_capture_name_eof_before_closing() {
    let mut parser = Parser::new("<group".to_string());
    let _ = parser.parse_capture_name(1).unwrap();
}

#[test]
fn test_parse_capture_name_invalid_char() {
    let mut parser = Parser::new("<gr@up>".to_string());
    let result = parser.parse_capture_name(1);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Error at 3: GroupNameInvalid");
}

#[test]
fn test_parse_capture_name_empty() {
    let mut parser = Parser::new("<>".to_string());
    let result = parser.parse_capture_name(1);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Error at 1: GroupNameEmpty");
}

