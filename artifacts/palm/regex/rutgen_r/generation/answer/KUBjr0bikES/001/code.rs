// Answer 0

#[test]
fn test_previous_char_valid_input() {
    struct TestInput {
        data: Vec<char>,
    }

    impl TestInput {
        fn previous_char(&self, at: InputAt) -> char {
            if at.index == 0 {
                panic!("Out of bounds: cannot access previous character at index 0");
            }
            self.data[at.index - 1]
        }
    }

    struct InputAt {
        index: usize,
    }

    let test_input = TestInput { data: vec!['a', 'b', 'c', 'd'] };
    let at = InputAt { index: 2 };
    
    let result = test_input.previous_char(at);
    assert_eq!(result, 'b');
}

#[test]
#[should_panic(expected = "Out of bounds: cannot access previous character at index 0")]
fn test_previous_char_panic_at_start() {
    struct TestInput {
        data: Vec<char>,
    }

    impl TestInput {
        fn previous_char(&self, at: InputAt) -> char {
            if at.index == 0 {
                panic!("Out of bounds: cannot access previous character at index 0");
            }
            self.data[at.index - 1]
        }
    }

    struct InputAt {
        index: usize,
    }

    let test_input = TestInput { data: vec!['x', 'y', 'z'] };
    let at = InputAt { index: 0 };
    
    let _ = test_input.previous_char(at);
}

#[test]
fn test_previous_char_boundary_case() {
    struct TestInput {
        data: Vec<char>,
    }

    impl TestInput {
        fn previous_char(&self, at: InputAt) -> char {
            if at.index == 0 {
                panic!("Out of bounds: cannot access previous character at index 0");
            }
            self.data[at.index - 1]
        }
    }

    struct InputAt {
        index: usize,
    }

    let test_input = TestInput { data: vec!['1', '2', '3', '4'] };
    let at = InputAt { index: 1 };
    
    let result = test_input.previous_char(at);
    assert_eq!(result, '1');
}

