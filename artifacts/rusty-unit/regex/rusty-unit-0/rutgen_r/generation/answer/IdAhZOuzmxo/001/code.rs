// Answer 0

#[test]
fn test_prefix_at_valid_case() {
    struct TestInput {
        content: String,
    }
    
    impl TestInput {
        fn at(&self, pos: usize) -> &str {
            &self.content[pos..]
        }
        
        fn len(&self) -> usize {
            self.content.len()
        }
    }
    
    struct LiteralSearcher {
        prefixes: Vec<String>,
    }
    
    impl LiteralSearcher {
        fn find(&self, text: &str) -> Option<(String, usize)> {
            for prefix in &self.prefixes {
                if text.starts_with(prefix) {
                    return Some(prefix.clone().into());
                }
            }
            None
        }
    }
    
    let input = TestInput {
        content: String::from("hello world"),
    };
    let prefixes = LiteralSearcher {
        prefixes: vec![String::from("hello"), String::from("world")],
    };
    let at = InputAt { pos: 0 };
    
    let result = input.prefix_at(&prefixes, at);
    
    assert_eq!(result.map(|pos| pos.pos()), Some(5)); // 'hello' has a length of 5
}

#[test]
#[should_panic]
fn test_prefix_at_panic_case() {
    struct TestInput {
        content: String,
    }
    
    impl TestInput {
        fn at(&self, pos: usize) -> &str {
            &self.content[pos..]
        }
        
        fn len(&self) -> usize {
            self.content.len()
        }
    }
    
    struct LiteralSearcher {
        prefixes: Vec<String>,
    }
    
    impl LiteralSearcher {
        fn find(&self, text: &str) -> Option<(String, usize)> {
            for prefix in &self.prefixes {
                if text.starts_with(prefix) {
                    return Some(prefix.clone().into());
                }
            }
            None
        }
    }
    
    let input = TestInput {
        content: String::from("hi"),
    };
    let prefixes = LiteralSearcher {
        prefixes: vec![String::from("hello")],
    };
    let at = InputAt { pos: 10 }; // out-of-bounds position
    
    let _result = input.prefix_at(&prefixes, at); // This should panic
}

