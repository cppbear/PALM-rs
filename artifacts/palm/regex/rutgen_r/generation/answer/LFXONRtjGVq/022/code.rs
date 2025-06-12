// Answer 0

#[test]
#[should_panic]
fn test_parse_counted_repetition_invalid_char_at_start() {
    struct MockParser {
        chars: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.chars[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.bump();
            true
        }

        fn parse_decimal(&mut self) -> Result<usize, ()> {
            Ok(2) // Return a valid decimal
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.chars.len()
        }

        fn error(&self, _span: Span, _error: ast::ErrorKind) -> Result<ast::Concat, ()> {
            Err(())
        }
    }

    let parser = MockParser {
        chars: vec!['a', '{'], // Invalid start character 'a'
        pos: 0,
    };
    
    let concat = ast::Concat {
        asts: vec![ast::Ast::SomeAst], // Placeholder for some valid AST
    };
    
    parser.parse_counted_repetition(concat).unwrap();
}

#[test]
fn test_parse_counted_repetition_repetition_count_unclosed() {
    struct MockParser {
        chars: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.chars[self.pos]
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump(&mut self) {
            self.pos += 1;
        }

        fn bump_and_bump_space(&mut self) -> bool {
            self.bump();
            true
        }

        fn parse_decimal(&mut self) -> Result<usize, ()> {
            Ok(2) // Return a valid decimal
        }

        fn is_eof(&self) -> bool {
            false // Not EOF
        }

        fn error(&self, span: Span, error: ast::ErrorKind) -> Result<ast::Concat, ()> {
            if let ast::ErrorKind::RepetitionCountUnclosed = error {
                return Err(());
            }
            Ok(span)
        }
    }

    let parser = MockParser {
        chars: vec!['{', '2', ','], // Proper start but unbalanced
        pos: 0,
    };

    let concat = ast::Concat {
        asts: vec![ast::Ast::SomeAst], // Placeholder for some valid AST
    };

    assert!(parser.parse_counted_repetition(concat).is_err());
}

