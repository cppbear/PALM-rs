// Answer 0

#[test]
#[should_panic]
fn test_parse_octal_octal_disabled() {
    struct DummyParser {
        octal: bool,
        current_char: char,
        position: usize,
    }

    impl DummyParser {
        fn new(octal: bool, current_char: char, position: usize) -> Self {
            DummyParser { octal, current_char, position }
        }
        
        fn octal(&self) -> bool {
            self.octal
        }
        
        fn char(&self) -> char {
            self.current_char
        }
        
        fn pos(&self) -> usize {
            self.position
        }
        
        fn bump(&mut self) -> bool {
            self.position += 1;
            false
        }
        
        fn pattern(&self) -> &str {
            "0"
        }
    }

    struct DummyAst {
        parser: DummyParser,
    }

    impl DummyAst {
        fn new(parser: DummyParser) -> Self {
            DummyAst { parser }
        }
        
        fn parser(&self) -> &DummyParser {
            &self.parser
        }
        
        fn parse_octal(&self) -> ast::Literal {
            // Function body pasted directly from the original context
            use std::char;
            use std::u32;

            assert!(self.parser().octal);
            assert!('0' <= self.char() && self.char() <= '7');
            let start = self.pos();
            // Parse up to two more digits.
            while
                self.bump() &&
                '0' <= self.char() && self.char() <= '7' &&
                self.pos().offset - start.offset <= 2
            {}
            let end = self.pos();
            let octal = &self.pattern()[start.offset..end.offset];
            // Parsing the octal should never fail since the above guarantees a
            // valid number.
            let codepoint =
                u32::from_str_radix(octal, 8).expect("valid octal number");
            // The max value for 3 digit octal is 0777 = 511 and [0, 511] has no
            // invalid Unicode scalar values.
            let c = char::from_u32(codepoint).expect("Unicode scalar value");
            ast::Literal {
                span: Span::new(start, end),
                kind: ast::LiteralKind::Octal,
                c: c,
            }
        }
    }

    let parser = DummyParser::new(false, '0', 0);
    let ast_instance = DummyAst::new(parser);
    let _ = ast_instance.parse_octal();
}

