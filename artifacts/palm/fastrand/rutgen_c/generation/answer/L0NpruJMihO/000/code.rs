// Answer 0

#[test]
fn test_alphanumeric() {
    struct TestRng {
        alphanumeric_char: char,
    }

    impl TestRng {
        fn alphanumeric(&mut self) -> char {
            self.alphanumeric_char
        }
    }

    let expected = 'A'; // Example character
    let mut rng = TestRng { alphanumeric_char: expected };

    let result = with_rng(|r| r.alphanumeric());

    assert_eq!(result, expected);
}

#[test]
fn test_alphanumeric_lowercase() {
    struct TestRng {
        alphanumeric_char: char,
    }

    impl TestRng {
        fn alphanumeric(&mut self) -> char {
            self.alphanumeric_char
        }
    }

    let expected = 'z'; // Example character
    let mut rng = TestRng { alphanumeric_char: expected };

    let result = with_rng(|r| r.alphanumeric());

    assert_eq!(result, expected);
}

#[test]
fn test_alphanumeric_digit() {
    struct TestRng {
        alphanumeric_char: char,
    }

    impl TestRng {
        fn alphanumeric(&mut self) -> char {
            self.alphanumeric_char
        }
    }

    let expected = '5'; // Example character
    let mut rng = TestRng { alphanumeric_char: expected };

    let result = with_rng(|r| r.alphanumeric());

    assert_eq!(result, expected);
}

#[should_panic]
#[test]
fn test_alphanumeric_invalid() {
    struct TestRng {
        alphanumeric_char: char,
    }

    impl TestRng {
        fn alphanumeric(&mut self) -> char {
            '!' // Invalid alphanumeric char
        }
    }

    let mut rng = TestRng { alphanumeric_char: '!' };

    let result = with_rng(|r| r.alphanumeric());

    assert!(!result.is_alphanumeric()); // This should panic if alphanumeric is not respected
}

