// Answer 0

#[test]
fn test_prefix_at_with_valid_input() {
    struct TestInput {
        data: String,
    }

    impl TestInput {
        fn at(&self, pos: usize) -> InputAt {
            InputAt::new(pos)
        }
        
        fn len(&self) -> usize {
            self.data.len()
        }
        
        fn range(&self, start: usize, end: usize) -> &str {
            &self.data[start..end]
        }
    }
    
    let input = TestInput {
        data: String::from("hello world"),
    };
    
    let prefixes = LiteralSearcher::new("hello");
    let at = input.at(0);
    
    let result = prefix_at(&input, &prefixes, at);
    
    assert_eq!(result, Some(input.at(5))); // "hello" has length 5
}

#[test]
#[should_panic]
fn test_prefix_at_with_out_of_bounds_position() {
    struct TestInput {
        data: String,
    }

    impl TestInput {
        fn at(&self, pos: usize) -> InputAt {
            InputAt::new(pos)
        }

        fn len(&self) -> usize {
            self.data.len()
        }
        
        fn range(&self, start: usize, end: usize) -> &str {
            &self.data[start..end]
        }
    }

    let input = TestInput {
        data: String::from("hello world"),
    };

    let prefixes = LiteralSearcher::new("hello");
    let at = input.at(15); // Out of bounds position

    let _ = prefix_at(&input, &prefixes, at);
}

#[test]
fn test_prefix_at_with_partial_match() {
    struct TestInput {
        data: String,
    }

    impl TestInput {
        fn at(&self, pos: usize) -> InputAt {
            InputAt::new(pos)
        }
        
        fn len(&self) -> usize {
            self.data.len()
        }

        fn range(&self, start: usize, end: usize) -> &str {
            &self.data[start..end]
        }
    }

    let input = TestInput {
        data: String::from("hello world"),
    };
    
    let prefixes = LiteralSearcher::new("hel");
    let at = input.at(0);
    
    let result = prefix_at(&input, &prefixes, at);
    
    assert_eq!(result, Some(input.at(3))); // "hel" has length 3
}

#[test]
fn test_prefix_at_with_no_match() {
    struct TestInput {
        data: String,
    }

    impl TestInput {
        fn at(&self, pos: usize) -> InputAt {
            InputAt::new(pos)
        }
        
        fn len(&self) -> usize {
            self.data.len()
        }

        fn range(&self, start: usize, end: usize) -> &str {
            &self.data[start..end]
        }
    }

    let input = TestInput {
        data: String::from("hello world"),
    };

    let prefixes = LiteralSearcher::new("abc"); // No match
    let at = input.at(0);
    
    let result = prefix_at(&input, &prefixes, at);
    
    assert_eq!(result, None);
}

