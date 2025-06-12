// Answer 0

#[test]
fn test_maybe_parse_ascii_class_invalid_no_closing_bracket() {
    struct Parser {
        pattern: String,
        pos: usize,
    }

    impl Parser {
        fn new(pattern: &str) -> Self {
            Parser {
                pattern: pattern.to_string(),
                pos: 0,
            }
        }

        fn bump(&mut self) -> bool {
            self.pos += 1;
            self.pos <= self.pattern.len()
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos).unwrap_or('\0')
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.pattern.len()
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
        
        fn maybe_parse_ascii_class(&mut self) -> Option<()>
        {
            assert_eq!(self.char(), '[');
            let start = self.pos();
            let mut negated = false;
            
            if !self.bump() || self.char() != ':' {
                return None;
            }
            if !self.bump() {
                return None;
            }

            if self.char() == '^' {
                negated = true;
                if !self.bump() {
                    return None;
                }
            }

            while self.char() != ':' && self.bump() {}
            if self.is_eof() {
                return None;
            }

            if !self.bump() {
                return None;
            }

            Some(())
        }
    }

    let mut parser = Parser::new("[[:alnum]");
    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_none());
}

#[test]
fn test_maybe_parse_ascii_class_invalid_missing_starting_colon() {
    struct Parser {
        pattern: String,
        pos: usize,
    }

    impl Parser {
        fn new(pattern: &str) -> Self {
            Parser {
                pattern: pattern.to_string(),
                pos: 0,
            }
        }

        fn bump(&mut self) -> bool {
            self.pos += 1;
            self.pos <= self.pattern.len()
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos).unwrap_or('\0')
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.pattern.len()
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
        
        fn maybe_parse_ascii_class(&mut self) -> Option<()>
        {
            assert_eq!(self.char(), '[');
            let start = self.pos();
            let mut negated = false;
            
            if !self.bump() || self.char() != ':' {
                return None;
            }
            if !self.bump() {
                return None;
            }

            if self.char() == '^' {
                negated = true;
                if !self.bump() {
                    return None;
                }
            }

            while self.char() != ':' && self.bump() {}
            if self.is_eof() {
                return None;
            }

            if !self.bump() {
                return None;
            }

            Some(())
        }
    }

    let mut parser = Parser::new("[alnum:]");
    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_none());
} 

#[test]
fn test_maybe_parse_ascii_class_invalid_no_name() {
    struct Parser {
        pattern: String,
        pos: usize,
    }

    impl Parser {
        fn new(pattern: &str) -> Self {
            Parser {
                pattern: pattern.to_string(),
                pos: 0,
            }
        }

        fn bump(&mut self) -> bool {
            self.pos += 1;
            self.pos <= self.pattern.len()
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos).unwrap_or('\0')
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.pattern.len()
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
        
        fn maybe_parse_ascii_class(&mut self) -> Option<()>
        {
            assert_eq!(self.char(), '[');
            let start = self.pos();
            let mut negated = false;
            
            if !self.bump() || self.char() != ':' {
                return None;
            }
            if !self.bump() {
                return None;
            }

            if self.char() == '^' {
                negated = true;
                if !self.bump() {
                    return None;
                }
            }

            while self.char() != ':' && self.bump() {}
            if self.is_eof() {
                return None;
            }

            if !self.bump() {
                return None;
            }

            Some(())
        }
    }

    let mut parser = Parser::new("[[:]:]");
    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_none());
} 

#[test]
fn test_maybe_parse_ascii_class_invalid_name_with_extra_colon() {
    struct Parser {
        pattern: String,
        pos: usize,
    }

    impl Parser {
        fn new(pattern: &str) -> Self {
            Parser {
                pattern: pattern.to_string(),
                pos: 0,
            }
        }

        fn bump(&mut self) -> bool {
            self.pos += 1;
            self.pos <= self.pattern.len()
        }

        fn char(&self) -> char {
            self.pattern.chars().nth(self.pos).unwrap_or('\0')
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn is_eof(&self) -> bool {
            self.pos >= self.pattern.len()
        }

        fn pattern(&self) -> &str {
            &self.pattern
        }
        
        fn maybe_parse_ascii_class(&mut self) -> Option<()>
        {
            assert_eq!(self.char(), '[');
            let start = self.pos();
            let mut negated = false;
            
            if !self.bump() || self.char() != ':' {
                return None;
            }
            if !self.bump() {
                return None;
            }

            if self.char() == '^' {
                negated = true;
                if !self.bump() {
                    return None;
                }
            }

            while self.char() != ':' && self.bump() {}
            if self.is_eof() {
                return None;
            }

            if !self.bump() {
                return None;
            }

            Some(())
        }
    }

    let mut parser = Parser::new("[[:loower:]]");
    let result = parser.maybe_parse_ascii_class();
    assert!(result.is_none());
}

