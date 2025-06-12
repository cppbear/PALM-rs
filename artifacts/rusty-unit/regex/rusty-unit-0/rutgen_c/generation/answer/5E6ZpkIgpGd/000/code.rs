// Answer 0

#[test]
fn test_next_char_valid_position() {
    struct TestInput {
        data: Vec<u8>,
    }
    
    impl<'t> Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt {
                pos: i,
                c: Char(self.data[i] as u32),
                byte: Some(self.data[i]),
                len: self.data.len(),
            }
        }
        
        fn next_char(&self, at: InputAt) -> Char {
            at.char()
        }
        
        fn previous_char(&self, at: InputAt) -> Char {
            Char(0)
        }
        
        fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool { false }
        
        fn prefix_at(&self, prefixes: &LiteralSearcher, at: InputAt) -> Option<InputAt> { None }
        
        fn len(&self) -> usize {
            self.data.len()
        }
        
        fn as_bytes(&self) -> &[u8] {
            &self.data
        }
    }

    let input_data = TestInput { data: b"abc".to_vec() };
    let at = input_data.at(1);
    let result = input_data.next_char(at);
    assert_eq!(result, Char(b'b' as u32));
}

#[test]
fn test_next_char_boundary_position_start() {
    struct TestInput {
        data: Vec<u8>,
    }

    impl<'t> Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt {
                pos: i,
                c: Char(self.data[i] as u32),
                byte: Some(self.data[i]),
                len: self.data.len(),
            }
        }
        
        fn next_char(&self, at: InputAt) -> Char {
            at.char()
        }
        
        fn previous_char(&self, at: InputAt) -> Char {
            Char(0)
        }
        
        fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool { false }
        
        fn prefix_at(&self, prefixes: &LiteralSearcher, at: InputAt) -> Option<InputAt> { None }
        
        fn len(&self) -> usize {
            self.data.len()
        }
        
        fn as_bytes(&self) -> &[u8] {
            &self.data
        }
    }

    let input_data = TestInput { data: b"abc".to_vec() };
    let at = input_data.at(0);
    let result = input_data.next_char(at);
    assert_eq!(result, Char(b'a' as u32));
}

#[test]
fn test_next_char_boundary_position_end() {
    struct TestInput {
        data: Vec<u8>,
    }

    impl<'t> Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt {
                pos: i,
                c: Char(self.data[i] as u32),
                byte: Some(self.data[i]),
                len: self.data.len(),
            }
        }
        
        fn next_char(&self, at: InputAt) -> Char {
            at.char()
        }
        
        fn previous_char(&self, at: InputAt) -> Char {
            Char(0)
        }
        
        fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool { false }
        
        fn prefix_at(&self, prefixes: &LiteralSearcher, at: InputAt) -> Option<InputAt> { None }
        
        fn len(&self) -> usize {
            self.data.len()
        }
        
        fn as_bytes(&self) -> &[u8] {
            &self.data
        }
    }

    let input_data = TestInput { data: b"abc".to_vec() };
    let at = input_data.at(2); // Last character
    let result = input_data.next_char(at);
    assert_eq!(result, Char(b'c' as u32));
}

