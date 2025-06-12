// Answer 0

#[test]
fn test_is_match_at_valid_start_zero() {
    struct RegexStruct;

    impl RegexStruct {
        fn shortest_match_at(&self, text: &str, start: usize) -> Option<&str> {
            if start > text.len() {
                None
            } else if text.starts_with("hello") && start == 0 {
                Some("hello")
            } else {
                None
            }
        }
    }

    let regex = RegexStruct;

    assert!(regex.is_match_at("hello world", 0));
    assert!(!regex.is_match_at("world hello", 0));
}

#[test]
fn test_is_match_at_valid_start_non_zero() {
    struct RegexStruct;

    impl RegexStruct {
        fn shortest_match_at(&self, text: &str, start: usize) -> Option<&str> {
            if start >= text.len() {
                None
            } else if text[start..].starts_with("world") {
                Some("world")
            } else {
                None
            }
        }
    }

    let regex = RegexStruct;

    assert!(regex.is_match_at("hello world", 6));
    assert!(!regex.is_match_at("hello world", 0));
}

#[test]
fn test_is_match_at_out_of_bounds() {
    struct RegexStruct;

    impl RegexStruct {
        fn shortest_match_at(&self, text: &str, start: usize) -> Option<&str> {
            if start >= text.len() {
                None
            } else {
                Some(text)
            }
        }
    }

    let regex = RegexStruct;

    assert!(!regex.is_match_at("hello world", 11)); // out of bounds
    assert!(!regex.is_match_at("hello world", 12)); // further out of bounds
}

#[test]
#[should_panic]
fn test_is_match_at_panic_for_exceeding_start() {
    struct RegexStruct;

    impl RegexStruct {
        fn shortest_match_at(&self, text: &str, start: usize) -> Option<&str> {
            if start > text.len() {
                panic!("Start is beyond text length");
            }
            Some(text)
        }
    }

    let regex = RegexStruct;

    regex.is_match_at("abc", 4); // should panic here
}

