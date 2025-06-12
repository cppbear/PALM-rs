// Answer 0

#[test]
fn test_shortest_match_at_valid_start() {
    struct TestRegex(&'static str);
    
    impl TestRegex {
        fn searcher_str(&self) -> &str {
            self.0
        }
    }

    let regex = TestRegex("a*b");
    let text = "aaaab";
    let start = 0;
    let result = regex.shortest_match_at(text, start);
    assert_eq!(result, Some(4)); // "aaaab" shortest match at index 4
}

#[test]
fn test_shortest_match_at_offset_zero() {
    struct TestRegex(&'static str);
    
    impl TestRegex {
        fn searcher_str(&self) -> &str {
            self.0
        }
    }

    let regex = TestRegex("^a"); // Anchor match
    let text = "abcd";
    let start = 0;
    let result = regex.shortest_match_at(text, start);
    assert_eq!(result, Some(0)); // Match found at the start
}

#[test]
fn test_shortest_match_at_no_match() {
    struct TestRegex(&'static str);
    
    impl TestRegex {
        fn searcher_str(&self) -> &str {
            self.0
        }
    }

    let regex = TestRegex("c*");
    let text = "abcd";
    let start = 0;
    let result = regex.shortest_match_at(text, start);
    assert_eq!(result, Some(0)); // Shortest match is empty, so it matches up to index 0
}

#[test]
fn test_shortest_match_at_invalid_start() {
    struct TestRegex(&'static str);
    
    impl TestRegex {
        fn searcher_str(&self) -> &str {
            self.0
        }
    }

    let regex = TestRegex("a*");
    let text = "abcd";
    let start = 10; // Invalid start, beyond the length of the text
    let result = regex.shortest_match_at(text, start);
    assert_eq!(result, None); // No match because start exceeds text length
}

#[test]
#[should_panic]
fn test_shortest_match_at_negative_start() {
    struct TestRegex(&'static str);
    
    impl TestRegex {
        fn searcher_str(&self) -> &str {
            self.0
        }
    }

    let regex = TestRegex("ab");
    let text = "abcd";
    let start = usize::MAX; // Simulates a negative start due to underflow
    let _result = regex.shortest_match_at(text, start);
}

