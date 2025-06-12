// Answer 0

#[test]
fn test_next_char_valid_input() {
    struct TestInput {
        data: Vec<u8>,
    }

    impl TestInput {
        fn at(&self, pos: usize) -> InputAt {
            InputAt::new(pos) // Assuming InputAt::new method exists
        }

        fn get(&self, at: InputAt) -> Char {
            next_char(self, at) // Directly calling the function
        }
    }

    let valid_input = TestInput {
        data: b"hello".to_vec(),
    };

    let result = valid_input.get(valid_input.at(0));
    assert_eq!(result, 'h');
}

#[test]
fn test_next_char_empty_input() {
    struct TestInput {
        data: Vec<u8>,
    }

    impl TestInput {
        fn at(&self, pos: usize) -> InputAt {
            InputAt::new(pos)
        }

        fn get(&self, at: InputAt) -> Char {
            next_char(self, at)
        }
    }

    let empty_input = TestInput {
        data: b"".to_vec(),
    };

    let result = empty_input.get(empty_input.at(0)); // This should panic
    panic!("Expected a panic here, but got: {:?}", result);
}

#[test]
#[should_panic]
fn test_next_char_out_of_bounds() {
    struct TestInput {
        data: Vec<u8>,
    }

    impl TestInput {
        fn at(&self, pos: usize) -> InputAt {
            InputAt::new(pos)
        }

        fn get(&self, at: InputAt) -> Char {
            next_char(self, at)
        }
    }

    let input = TestInput {
        data: b"world".to_vec(),
    };

    let _result = input.get(input.at(10)); // Accessing out of bounds, should panic
}

#[test]
fn test_next_char_valid_utf8() {
    struct TestInput {
        data: Vec<u8>,
    }

    impl TestInput {
        fn at(&self, pos: usize) -> InputAt {
            InputAt::new(pos)
        }

        fn get(&self, at: InputAt) -> Char {
            next_char(self, at)
        }
    }

    let valid_utf8_input = TestInput {
        data: "こんにちは".as_bytes().to_vec(),
    };

    let result = valid_utf8_input.get(valid_utf8_input.at(0));
    assert_eq!(result, 'こ');
}

