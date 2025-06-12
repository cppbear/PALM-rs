// Answer 0

#[test]
fn test_at_function_valid_index() {
    struct TestInput {
        data: Vec<char>,
    }

    impl Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            let c = self.data[i];
            InputAt {
                pos: i,
                c: c,
                byte: Some(c as u8),
                len: self.data.len(),
            }
        }
        
        fn next_char(&self, _: InputAt) -> Char {
            // Implementation not needed for this test
            unimplemented!()
        }
        
        fn previous_char(&self, _: InputAt) -> Char {
            // Implementation not needed for this test
            unimplemented!()
        }

        fn is_empty_match(&self, _: InputAt, _: &InstEmptyLook) -> bool {
            // Implementation not needed for this test
            unimplemented!()
        }

        fn prefix_at(&self, _: &LiteralSearcher, _: InputAt) -> Option<InputAt> {
            // Implementation not needed for this test
            unimplemented!()
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn as_bytes(&self) -> &[u8] {
            unimplemented!()
        }
    }

    let input = TestInput {
        data: vec!['a', 'b', 'c'],
    };

    let result = input.at(1);
    assert_eq!(result.pos, 1);
    assert_eq!(result.c, 'b');
    assert_eq!(result.byte, Some(b'b'));
    assert_eq!(result.len, 3);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_at_function_out_of_bounds() {
    struct TestInput {
        data: Vec<char>,
    }

    impl Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            self.data[i]  // This will panic if i is out of bounds
        }
        
        fn next_char(&self, _: InputAt) -> Char {
            unimplemented!()
        }
        
        fn previous_char(&self, _: InputAt) -> Char {
            unimplemented!()
        }

        fn is_empty_match(&self, _: InputAt, _: &InstEmptyLook) -> bool {
            unimplemented!()
        }

        fn prefix_at(&self, _: &LiteralSearcher, _: InputAt) -> Option<InputAt> {
            unimplemented!()
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn as_bytes(&self) -> &[u8] {
            unimplemented!()
        }
    }

    let input = TestInput {
        data: vec!['a', 'b', 'c'],
    };

    // This will panic because the index 5 is out of bounds
    input.at(5);
}

