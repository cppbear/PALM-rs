// Answer 0

#[test]
fn test_next_char_valid() {
    #[derive(Default)]
    struct TestInput<'t> {
        data: &'t [u8],
    }

    impl<'t> Input for TestInput<'t> {
        fn at(&self, i: usize) -> InputAt {
            InputAt {
                pos: i,
                c: Char(u32::from(self.data[i])),
                byte: Some(self.data[i]),
                len: self.data.len(),
            }
        }

        fn next_char(&self, at: InputAt) -> Char {
            at.char()
        }

        fn previous_char(&self, at: InputAt) -> Char { unimplemented!() }
        fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool { unimplemented!() }
        fn prefix_at(&self, prefixes: &LiteralSearcher, at: InputAt) -> Option<InputAt> { unimplemented!() }
        fn len(&self) -> usize { self.data.len() }
        fn as_bytes(&self) -> &[u8] { self.data }
    }

    let data = b"hello";
    let test_input = TestInput { data: data };

    let at = test_input.at(1);
    let result_char = test_input.next_char(at);
    assert_eq!(result_char, Char(101)); // 'e'

    let at_end = test_input.at(data.len() - 1);
    let result_end_char = test_input.next_char(at_end);
    assert_eq!(result_end_char, Char(111)); // 'o'
}

#[test]
#[should_panic]
fn test_next_char_out_of_bounds() {
    #[derive(Default)]
    struct TestInput<'t> {
        data: &'t [u8],
    }

    impl<'t> Input for TestInput<'t> {
        fn at(&self, i: usize) -> InputAt {
            InputAt {
                pos: i,
                c: Char(u32::from(self.data[i])),
                byte: Some(self.data[i]),
                len: self.data.len(),
            }
        }

        fn next_char(&self, at: InputAt) -> Char {
            at.char()
        }

        fn previous_char(&self, at: InputAt) -> Char { unimplemented!() }
        fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool { unimplemented!() }
        fn prefix_at(&self, prefixes: &LiteralSearcher, at: InputAt) -> Option<InputAt> { unimplemented!() }
        fn len(&self) -> usize { self.data.len() }
        fn as_bytes(&self) -> &[u8] { self.data }
    }

    let data = b"test";
    let test_input = TestInput { data: data };

    // Attempting to access character at a position out of bounds
    let at_out_of_bounds = test_input.at(data.len());
    test_input.next_char(at_out_of_bounds); // This should panic
}

