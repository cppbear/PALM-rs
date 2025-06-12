// Answer 0

fn bump_and_bump_space_test() {
    struct Parser {
        position: usize,
        input: Vec<char>,
    }

    impl Parser {
        fn bump(&mut self) -> bool {
            if self.position < self.input.len() {
                self.position += 1;
                true
            } else {
                false
            }
        }

        fn bump_space(&mut self) {
            while self.position < self.input.len() && self.input[self.position].is_whitespace() {
                self.position += 1;
            }
        }

        fn is_eof(&self) -> bool {
            self.position >= self.input.len()
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if !self.bump() {
                return false;
            }
            self.bump_space();
            !self.is_eof()
        }
    }

    // Test case where self.bump() returns false
    {
        let mut parser = Parser {
            position: 0,
            input: vec![], // Empty input means bump will return false
        };
        
        assert_eq!(parser.bump_and_bump_space(), false);
    }

    // Additional edge case to confirm behavior
    {
        let mut parser = Parser {
            position: 0,
            input: vec![' '], // Single space character
        };

        assert_eq!(parser.bump_and_bump_space(), false); // Initial bump will succeed, but will then bump to EOF
    }
}

