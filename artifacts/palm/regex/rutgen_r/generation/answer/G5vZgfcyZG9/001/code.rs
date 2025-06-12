// Answer 0

#[derive(Debug)]
struct TestPattern {
    pattern: String,
}

impl TestPattern {
    fn new(pattern: &str) -> Self {
        TestPattern {
            pattern: pattern.to_string(),
        }
    }

    fn pattern(&self) -> &str {
        &self.pattern
    }

    fn char_at(&self, i: usize) -> char {
        self.pattern()[i..].chars().next()
            .unwrap_or_else(|| {
                panic!("expected char at offset {}", i)
            })
    }
}

#[test]
fn test_char_at_valid_position() {
    let test_pattern = TestPattern::new("hello");
    assert_eq!(test_pattern.char_at(0), 'h');
    assert_eq!(test_pattern.char_at(1), 'e');
    assert_eq!(test_pattern.char_at(2), 'l');
    assert_eq!(test_pattern.char_at(3), 'l');
    assert_eq!(test_pattern.char_at(4), 'o');
}

#[test]
#[should_panic(expected = "expected char at offset 5")]
fn test_char_at_out_of_bounds() {
    let test_pattern = TestPattern::new("hello");
    let _ = test_pattern.char_at(5);
}

#[test]
#[should_panic(expected = "expected char at offset 0")]
fn test_char_at_empty_pattern() {
    let test_pattern = TestPattern::new("");
    let _ = test_pattern.char_at(0);
}

