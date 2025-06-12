// Answer 0

#[test]
fn test_text_returns_correct_value() {
    struct MockRegex {
        text: String,
    }
    
    impl RegularExpression for MockRegex {
        type Text = String;

        fn some_method(&self) {} 
    }
    
    let regex = MockRegex {
        text: String::from("Hello, World!"),
    };
    let text_ref = &regex.text;

    let matches = Matches {
        re: regex,
        text: text_ref,
        last_end: 0,
        last_match: None,
    };

    assert_eq!(matches.text(), text_ref);
}

#[test]
fn test_text_empty_string() {
    struct MockRegex {
        text: String,
    }
    
    impl RegularExpression for MockRegex {
        type Text = String;

        fn some_method(&self) {}
    }

    let regex = MockRegex {
        text: String::from(""),
    };
    let text_ref = &regex.text;

    let matches = Matches {
        re: regex,
        text: text_ref,
        last_end: 0,
        last_match: None,
    };

    assert_eq!(matches.text(), text_ref);
}

#[test]
fn test_text_large_string() {
    struct MockRegex {
        text: String,
    }

    impl RegularExpression for MockRegex {
        type Text = String;

        fn some_method(&self) {}
    }

    let large_text = "a".repeat(10000);
    let regex = MockRegex {
        text: large_text.clone(),
    };
    let text_ref = &regex.text;

    let matches = Matches {
        re: regex,
        text: text_ref,
        last_end: 0,
        last_match: None,
    };

    assert_eq!(matches.text(), text_ref);
}

