// Answer 0

#[test]
fn test_previous_char_valid() {
    struct TestInput {
        data: Vec<u32>,
    }

    impl Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt {
                pos: i,
                c: Char(self.data[i]),
                byte: None,
                len: self.len(),
            }
        }
        
        fn next_char(&self, at: InputAt) -> Char {
            self.at(at.pos + 1).c
        }
        
        fn previous_char(&self, at: InputAt) -> Char {
            self.at(at.pos - 1).c
        }

        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool {
            false
        }
        
        fn prefix_at(&self, _prefixes: &LiteralSearcher, _at: InputAt) -> Option<InputAt> {
            None
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn as_bytes(&self) -> &[u8] {
            &[]
        }
    }

    let input = TestInput { data: vec![97, 98, 99] }; // characters 'a', 'b', 'c'
    let at = input.at(1); // pointing to 'b'
    let char_before = input.previous_char(at);
    
    assert_eq!(char_before, Char(97)); // should be 'a'
}

#[test]
#[should_panic]
fn test_previous_char_out_of_bounds() {
    struct TestInput {
        data: Vec<u32>,
    }

    impl Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt {
                pos: i,
                c: Char(self.data[i]),
                byte: None,
                len: self.len(),
            }
        }
        
        fn next_char(&self, at: InputAt) -> Char {
            self.at(at.pos + 1).c
        }
        
        fn previous_char(&self, at: InputAt) -> Char {
            self.at(at.pos - 1).c
        }

        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool {
            false
        }
        
        fn prefix_at(&self, _prefixes: &LiteralSearcher, _at: InputAt) -> Option<InputAt> {
            None
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn as_bytes(&self) -> &[u8] {
            &[]
        }
    }

    let input = TestInput { data: vec![97, 98, 99] }; // 'a', 'b', 'c'
    let at = input.at(0); // pointing to 'a'
    let _char_before = input.previous_char(at); // should panic when trying to go before the first character
}

