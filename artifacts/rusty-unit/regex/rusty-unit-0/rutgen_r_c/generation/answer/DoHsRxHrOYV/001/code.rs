// Answer 0

#[test]
fn test_is_empty_match_non_empty() {
    struct TestInput {
        data: Vec<u8>,
    }
    
    impl Input for TestInput {
        fn at(&self, _i: usize) -> InputAt {
            InputAt { pos: 0, c: Char::from('\0'), byte: None, len: 0 }
        }
        
        fn next_char(&self, _at: InputAt) -> Char {
            Char::from('\0')
        }

        fn previous_char(&self, _at: InputAt) -> Char {
            Char::from('\0')
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
            &self.data
        }
    }

    let input = TestInput { data: vec![1, 2, 3] };
    let empty_look = InstEmptyLook { goto: InstPtr::default(), look: EmptyLook::default() };
    let result = input.is_empty_match(InputAt { pos: 0, c: Char::from('a'), byte: None, len: 1 }, &empty_look);
    assert_eq!(result, false);
}

#[test]
fn test_is_empty_match_empty() {
    struct TestInput {
        data: Vec<u8>,
    }
    
    impl Input for TestInput {
        fn at(&self, _i: usize) -> InputAt {
            InputAt { pos: 0, c: Char::from('\0'), byte: None, len: 0 }
        }
        
        fn next_char(&self, _at: InputAt) -> Char {
            Char::from('\0')
        }

        fn previous_char(&self, _at: InputAt) -> Char {
            Char::from('\0')
        }

        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool {
            true
        }

        fn prefix_at(&self, _prefixes: &LiteralSearcher, _at: InputAt) -> Option<InputAt> {
            None
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn as_bytes(&self) -> &[u8] {
            &self.data
        }
    }

    let input = TestInput { data: vec![] };
    let empty_look = InstEmptyLook { goto: InstPtr::default(), look: EmptyLook::default() };
    let result = input.is_empty_match(InputAt { pos: 0, c: Char::from('a'), byte: None, len: 1 }, &empty_look);
    assert_eq!(result, true);
}

