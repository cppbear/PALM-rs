// Answer 0

#[test]
fn test_matches_text() {
    struct DummyRegex;
    impl RegularExpression for DummyRegex {
        type Text = String;
    }

    let text_to_search = String::from("example text");
    let matches = Matches {
        re: DummyRegex,
        text: &text_to_search,
        last_end: 0,
        last_match: None,
    };

    assert_eq!(matches.text(), &text_to_search);
}

