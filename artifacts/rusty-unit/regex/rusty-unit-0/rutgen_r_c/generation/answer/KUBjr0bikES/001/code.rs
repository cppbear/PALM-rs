// Answer 0

#[test]
fn test_previous_char_valid_input() {
    struct TestInput {
        chars: Vec<Char>,
    }

    impl Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt {
                pos: i,
                c: self.chars[i],
                byte: None,
                len: self.chars.len(),
            }
        }

        fn next_char(&self, at: InputAt) -> Char {
            if at.pos + 1 < self.chars.len() {
                self.chars[at.pos + 1]
            } else {
                self.chars.last().cloned().unwrap()
            }
        }

        fn previous_char(&self, at: InputAt) -> Char {
            if at.pos > 0 {
                self.chars[at.pos - 1]
            } else {
                self.chars[0]
            }
        }

        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool {
            false
        }

        fn prefix_at(&self, _prefixes: &LiteralSearcher, at: InputAt) -> Option<InputAt> {
            Some(at)
        }

        fn len(&self) -> usize {
            self.chars.len()
        }

        fn as_bytes(&self) -> &[u8] {
            unimplemented!()
        }
    }
    
    let input = TestInput {
        chars: vec![Char(97), Char(98), Char(99)], // 'a', 'b', 'c'
    };
    let input_ref: &TestInput = &input;

    let at_b = input_ref.at(1);
    let prev_char = input_ref.previous_char(at_b);
    assert_eq!(prev_char, Char(97)); // Expecting 'a'
}

#[test]
fn test_previous_char_boundary_condition_start() {
    struct TestInput {
        chars: Vec<Char>,
    }

    impl Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt {
                pos: i,
                c: self.chars[i],
                byte: None,
                len: self.chars.len(),
            }
        }

        fn next_char(&self, at: InputAt) -> Char {
            if at.pos + 1 < self.chars.len() {
                self.chars[at.pos + 1]
            } else {
                self.chars.last().cloned().unwrap()
            }
        }

        fn previous_char(&self, at: InputAt) -> Char {
            if at.pos > 0 {
                self.chars[at.pos - 1]
            } else {
                self.chars[0]
            }
        }

        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool {
            false
        }

        fn prefix_at(&self, _prefixes: &LiteralSearcher, at: InputAt) -> Option<InputAt> {
            Some(at)
        }

        fn len(&self) -> usize {
            self.chars.len()
        }

        fn as_bytes(&self) -> &[u8] {
            unimplemented!()
        }
    }
    
    let input = TestInput {
        chars: vec![Char(97)], // 'a'
    };
    let input_ref: &TestInput = &input;

    let at_a = input_ref.at(0);
    let prev_char = input_ref.previous_char(at_a);
    assert_eq!(prev_char, Char(97)); // Expecting 'a'
}

#[test]
fn test_previous_char_empty_input() {
    struct TestInput {
        chars: Vec<Char>,
    }

    impl Input for TestInput {
        fn at(&self, _i: usize) -> InputAt {
            InputAt {
                pos: 0,
                c: Char(0),
                byte: None,
                len: 0,
            }
        }

        fn next_char(&self, _at: InputAt) -> Char {
            Char(0)
        }

        fn previous_char(&self, at: InputAt) -> Char {
            Char(0) // Returns a dummy Char for empty input
        }

        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool {
            false
        }

        fn prefix_at(&self, _prefixes: &LiteralSearcher, at: InputAt) -> Option<InputAt> {
            Some(at)
        }

        fn len(&self) -> usize {
            0
        }

        fn as_bytes(&self) -> &[u8] {
            unimplemented!()
        }
    }
    
    let input = TestInput {
        chars: vec![],
    };
    let input_ref: &TestInput = &input;

    let at_no_char = input_ref.at(0);
    let prev_char = input_ref.previous_char(at_no_char);
    assert_eq!(prev_char, Char(0)); // Expecting default Char(0) for empty input
}

