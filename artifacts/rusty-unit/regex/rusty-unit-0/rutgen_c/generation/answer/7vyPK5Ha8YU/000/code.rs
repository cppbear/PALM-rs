// Answer 0

#[test]
fn test_regex() {
    struct DummyRegex;
    
    impl DummyRegex {
        type Text = String;
    }

    struct DummyRegularExpression;

    impl RegularExpression for DummyRegularExpression {
        type Text = String;
    }

    let regex = DummyRegularExpression;
    let text = String::from("test text");
    let matches = Matches {
        re: regex,
        text: &text,
        last_end: 0,
        last_match: None,
    };

    let returned_regex = matches.regex();
    assert!(std::ptr::eq(returned_regex, &matches.re));
}

