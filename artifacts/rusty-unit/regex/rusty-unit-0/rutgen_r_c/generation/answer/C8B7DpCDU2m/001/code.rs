// Answer 0

#[test]
fn test_capture_matches_regex() {
    struct TestRegex;
    
    impl RegularExpression for TestRegex {
        type Text = str;
    
        fn regex(&self) -> &Self {
            self
        }
    }

    let test_text: &str = "sample text";
    let matches = Matches {
        re: TestRegex,
        text: &test_text,
        last_end: 0,
        last_match: None,
    };
    
    let capture_matches = CaptureMatches(matches);
    assert_eq!(capture_matches.regex() as *const _ as usize, &TestRegex as *const _ as usize);
}

#[test]
fn test_matches_regex() {
    struct TestRegex;

    impl RegularExpression for TestRegex {
        type Text = str;
    
        fn regex(&self) -> &Self {
            self
        }
    }

    let test_text: &str = "sample text";
    let matches = Matches {
        re: TestRegex,
        text: &test_text,
        last_end: 0,
        last_match: None,
    };
    
    assert_eq!(matches.regex() as *const _ as usize, &TestRegex as *const _ as usize);
}

