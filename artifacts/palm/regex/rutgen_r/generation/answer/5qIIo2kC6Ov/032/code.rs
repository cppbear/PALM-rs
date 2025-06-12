// Answer 0

#[test]
#[should_panic]
fn test_parse_with_comments_should_panic_on_unmatched_group() {
    struct Parser {
        offset: usize,
        input: Vec<char>,
        index: usize,
        comments: Vec<String>,
    }

    impl Parser {
        fn new(input: &str, comments: Vec<String>) -> Self {
            Self {
                offset: 0,
                input: input.chars().collect(),
                index: 0,
                comments,
            }
        }

        fn bump_space(&mut self) {
            // Simulate space bumping by just advancing the index
            if self.index < self.input.len() {
                self.index += 1;
            }
        }

        fn is_eof(&self) -> bool {
            self.index >= self.input.len()
        }

        fn char(&self) -> char {
            self.input[self.index]
        }

        fn pop_group(&self, _concat: ast::Concat) -> Result<ast::Concat, ()> {
            // Simulate a situation where popping a group fails
            Err(())
        }

        fn parser(&self) -> ParserRef {
            ParserRef {
                comments: &self.comments,
            }
        }

        fn offset(&self) -> usize {
            self.offset
        }

        fn reset(&mut self) {
            self.index = 0;
        }

        fn span(&self) -> usize {
            // Simulated span return
            self.index
        }
    }

    struct ParserRef<'a> {
        comments: &'a Vec<String>,
    }

    impl ParserRef<'_> {
        fn borrow_mut(&self) -> &Vec<String> {
            self.comments
        }
    }

    let parser = Parser::new("((a)b)", vec!["comment1".to_string(), "comment2".to_string()]);
    parser.parse_with_comments().unwrap();
}

#[test]
fn test_parse_with_comments_valid_input() {
    struct Parser {
        offset: usize,
        input: Vec<char>,
        index: usize,
        comments: Vec<String>,
    }

    impl Parser {
        fn new(input: &str, comments: Vec<String>) -> Self {
            Self {
                offset: 0,
                input: input.chars().collect(),
                index: 0,
                comments,
            }
        }

        fn bump_space(&mut self) {
            // Simulate space bumping by just advancing the index
            if self.index < self.input.len() {
                self.index += 1;
            }
        }

        fn is_eof(&self) -> bool {
            self.index >= self.input.len()
        }

        fn char(&self) -> char {
            self.input[self.index]
        }

        fn push_group(&self, concat: ast::Concat) -> Result<ast::Concat, ()> {
            // Simulate a valid push_group
            Ok(concat) // Pretend it works
        }

        fn pop_group(&self, concat: ast::Concat) -> Result<ast::Concat, ()> {
            // Simulate a valid pop_group
            Ok(concat) // Pretend it works
        }

        fn parser(&self) -> ParserRef {
            ParserRef {
                comments: &self.comments,
            }
        }

        fn offset(&self) -> usize {
            self.offset
        }

        fn reset(&mut self) {
            self.index = 0;
        }

        fn span(&self) -> usize {
            // Simulated span return
            self.index
        }
        
        fn parse_with_comments(&self) -> Result<ast::WithComments, ()> {
            assert_eq!(self.offset(), 0, "parser can only be used once");
            self.reset();
            let mut concat = ast::Concat {
                span: self.span(),
                asts: vec![],
            };
            while !self.is_eof() {
                self.bump_space();
                match self.char() {
                    '(' => concat = self.push_group(concat)?,
                    ')' => concat = self.pop_group(concat)?,
                    _ => {}
                }
            }
            Ok(ast::WithComments {
                ast: concat,
                comments: self.comments.clone(),
            })
        }
    }

    struct ParserRef<'a> {
        comments: &'a Vec<String>,
    }

    impl ParserRef<'_> {
        fn borrow_mut(&self) -> &Vec<String> {
            self.comments
        }
    }

    let parser = Parser::new("()", vec!["comment".to_string()]);
    let result = parser.parse_with_comments().unwrap();
    assert_eq!(result.comments.len(), 1);
}

