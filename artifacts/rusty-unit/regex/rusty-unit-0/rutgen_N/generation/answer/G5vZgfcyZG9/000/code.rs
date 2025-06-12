// Answer 0

#[test]
fn test_char_at_valid_position() {
    struct TestPattern {
        pattern: String,
    }

    impl TestPattern {
        fn pattern(&self) -> &str {
            &self.pattern
        }
    }

    let test_case = TestPattern { pattern: String::from("hello") };
    assert_eq!(test_case.char_at(0), 'h');
    assert_eq!(test_case.char_at(1), 'e');
    assert_eq!(test_case.char_at(2), 'l');
    assert_eq!(test_case.char_at(3), 'l');
    assert_eq!(test_case.char_at(4), 'o');
}

#[test]
#[should_panic(expected = "expected char at offset 5")]
fn test_char_at_invalid_position() {
    struct TestPattern {
        pattern: String,
    }

    impl TestPattern {
        fn pattern(&self) -> &str {
            &self.pattern
        }
    }

    let test_case = TestPattern { pattern: String::from("hello") };
    test_case.char_at(5);
}

