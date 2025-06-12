// Answer 0

#[test]
fn test_is_match_at() {
    struct Matcher(Vec<u8>);
    
    impl Matcher {
        fn is_match_at(&self, text: &[u8], start: usize) -> bool {
            let text_len = text.len();
            if start > text_len {
                panic!("start index out of bounds");
            }
            self.0.iter().zip(&text[start..]).all(|(m, t)| m == t)
        }

        fn new(pattern: &str) -> Self {
            Matcher(pattern.as_bytes().to_vec())
        }
    }

    let matcher = Matcher::new("abc");

    // Test matching at the start of the string
    assert!(matcher.is_match_at(b"abcde", 0));
    // Test matching in the middle of the string
    assert!(matcher.is_match_at(b"XYZabcde", 3));
    // Test not matching
    assert!(!matcher.is_match_at(b"XYZdeabc", 3));
    
    // Test start index just past the end of the string
    let result = std::panic::catch_unwind(|| {
        matcher.is_match_at(b"abc", 4);
    });
    assert!(result.is_err());

    // Test start index equal to string length
    assert!(!matcher.is_match_at(b"abc", 3));
}

