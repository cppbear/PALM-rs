// Answer 0

#[derive(Debug)]
struct MockParser {
    input: Vec<char>,
    position: usize,
}

impl MockParser {
    fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn char(&self) -> char {
        self.input[self.position]
    }

    fn error(&self, _span: usize, _kind: ast::ErrorKind) -> String {
        "Unrecognized flag".to_string()
    }

    fn span_char(&self) -> usize {
        self.position
    }

    fn parse_flag(&self) -> Result<ast::Flag, String> {
        match self.char() {
            'i' => Ok(ast::Flag::CaseInsensitive),
            'm' => Ok(ast::Flag::MultiLine),
            's' => Ok(ast::Flag::DotMatchesNewLine),
            'U' => Ok(ast::Flag::SwapGreed),
            'u' => Ok(ast::Flag::Unicode),
            'x' => Ok(ast::Flag::IgnoreWhitespace),
            _ => Err(self.error(self.span_char(), ast::ErrorKind::FlagUnrecognized)),
        }
    }
}

mod ast {
    #[derive(Debug, PartialEq)]
    pub enum Flag {
        CaseInsensitive,
        MultiLine,
        DotMatchesNewLine,
        SwapGreed,
        Unicode,
        IgnoreWhitespace,
    }

    #[derive(Debug)]
    pub enum ErrorKind {
        FlagUnrecognized,
    }
}

#[test]
fn test_parse_flag_multiline() {
    let parser = MockParser::new("m");
    let result = parser.parse_flag();
    assert_eq!(result, Ok(ast::Flag::MultiLine));
}

#[test]
fn test_parse_flag_case_insensitive() {
    let parser = MockParser::new("i");
    let result = parser.parse_flag();
    assert_eq!(result, Ok(ast::Flag::CaseInsensitive));
}

#[test]
fn test_parse_flag_dot_matches_new_line() {
    let parser = MockParser::new("s");
    let result = parser.parse_flag();
    assert_eq!(result, Ok(ast::Flag::DotMatchesNewLine));
}

#[test]
fn test_parse_flag_swap_greed() {
    let parser = MockParser::new("U");
    let result = parser.parse_flag();
    assert_eq!(result, Ok(ast::Flag::SwapGreed));
}

#[test]
fn test_parse_flag_unicode() {
    let parser = MockParser::new("u");
    let result = parser.parse_flag();
    assert_eq!(result, Ok(ast::Flag::Unicode));
}

#[test]
fn test_parse_flag_ignore_whitespace() {
    let parser = MockParser::new("x");
    let result = parser.parse_flag();
    assert_eq!(result, Ok(ast::Flag::IgnoreWhitespace));
}

#[test]
fn test_parse_flag_unrecognized() {
    let parser = MockParser::new("a");
    let result = parser.parse_flag();
    assert!(result.is_err());
}

