// Answer 0

fn at_test() {
    struct TestInput(Option<Vec<char>>);

    impl Deref for TestInput {
        type Target = Vec<char>;

        fn deref(&self) -> &Self::Target {
            self.0.as_ref().map_or_else(|| &[][..], |v| v)
        }
    }

    struct InputAt {
        char: Option<char>,
    }

    impl TestInput {
        fn at(&self, i: usize) -> InputAt {
            if let Some(ref v) = self.0 {
                if i < v.len() {
                    InputAt { char: Some(v[i]) }
                } else {
                    InputAt { char: None }
                }
            } else {
                InputAt { char: None }
            }
        }
    }

    // Test when input vector is None
    let input = TestInput(None);
    let result = input.at(0);
    assert_eq!(result.char, None);

    // Test when accessing valid index
    let input = TestInput(Some(vec!['a', 'b', 'c']));
    let result = input.at(1);
    assert_eq!(result.char, Some('b'));

    // Test when accessing index equal to vector length
    let result = input.at(3);
    assert_eq!(result.char, None);

    // Test when accessing index greater than vector length
    let result = input.at(10);
    assert_eq!(result.char, None);
}

