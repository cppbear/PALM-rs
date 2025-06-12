// Answer 0

#[derive(Debug)]
struct MockParser {
    pos: usize,
    characters: Vec<char>,
}

impl MockParser {
    fn new(chars: Vec<char>) -> Self {
        MockParser { pos: 0, characters: chars }
    }

    fn char(&self) -> char {
        *self.characters.get(self.pos).unwrap_or(&'\0')
    }

    fn bump(&mut self) {
        self.pos += 1;
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.characters.len()
    }

    fn bump_and_bump_space(&mut self) -> bool {
        self.bump(); // simulate bump
        true // assume space
    }

    fn parse_decimal(&mut self) -> Result<usize, ()> {
        Ok(2) // return a fixed decimal for simplicity
    }

    fn span(&self) -> usize {
        self.pos
    }

    fn error(&self, _span: usize, _kind: ast::ErrorKind) -> Result<ast::Concat, ()> {
        Err(())
    }

    fn pos(&self) -> usize {
        self.pos
    }
}

#[derive(Debug)]
struct MockConcat {
    asts: Vec<ast::Ast>,
}

impl MockConcat {
    fn new(asts: Vec<ast::Ast>) -> Self {
        MockConcat { asts }
    }
}

#[test]
fn test_parse_counted_repetition_invalid_range() {
    let parser = MockParser::new(vec!['{', '2', ',', '3', '}', '?']);
    let mut concat = MockConcat::new(vec![ast::Ast {}]); // Assume a valid Ast

    let result = parse_counted_repetition(&parser, concat);
    
    assert!(result.is_err());
}

#[test]
fn test_parse_counted_repetition_unclosed_count() {
    let parser = MockParser::new(vec!['{', '2', ',', '}', '?']);
    let mut concat = MockConcat::new(vec![ast::Ast {}]); // Assume a valid Ast

    let result = parse_counted_repetition(&parser, concat);

    assert!(result.is_err());
}

#[test]
fn test_parse_counted_repetition_no_greedy() {
    let parser = MockParser::new(vec!['{', '2', ',', '3', '}', '?']);
    let mut concat = MockConcat::new(vec![ast::Ast {}]); // Assume a valid Ast

    let result = parse_counted_repetition(&parser, concat);
    
    assert!(result.is_err());
}

