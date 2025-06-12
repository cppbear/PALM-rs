// Answer 0

#[test]
fn test_capture_matches_text() {
    struct TestRegex;

    impl RegularExpression for TestRegex {
        type Text = String;
        // Implement other required associated types and methods here if necessary
    }

    let re = TestRegex;
    let text = String::from("Hello, World!");
    let last_end = 0;
    let last_match = None;

    let matches = Matches {
        re,
        text: &text,
        last_end,
        last_match,
    };
    let capture_matches = CaptureMatches(matches);

    assert_eq!(capture_matches.text(), "Hello, World!");
}

#[test]
fn test_matches_text() {
    struct TestRegex;

    impl RegularExpression for TestRegex {
        type Text = String;
        // Implement other required associated types and methods here if necessary
    }

    let re = TestRegex;
    let text = String::from("Test String");
    let last_end = 0;
    let last_match = None;

    let matches = Matches {
        re,
        text: &text,
        last_end,
        last_match,
    };

    assert_eq!(matches.text(), "Test String");
}

