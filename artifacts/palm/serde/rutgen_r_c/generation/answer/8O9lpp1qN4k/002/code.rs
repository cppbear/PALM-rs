// Answer 0

#[test]
fn test_unexpected_seq() {
    struct TestContent<'de> {
        content: Content<'de>,
    }

    let test_content = TestContent {
        content: Content::Seq(vec![
            Content::U8(1),
            Content::String("example".to_string()),
            Content::Bool(true),
        ]),
    };

    let result = test_content.content.unexpected();
    assert_eq!(result, Unexpected::Seq);
}

#[test]
fn test_unexpected_seq_empty() {
    struct TestContent<'de> {
        content: Content<'de>,
    }

    let test_content = TestContent {
        content: Content::Seq(vec![]),
    };

    let result = test_content.content.unexpected();
    assert_eq!(result, Unexpected::Seq);
}

#[test]
fn test_unexpected_seq_nested() {
    struct TestContent<'de> {
        content: Content<'de>,
    }

    let test_content = TestContent {
        content: Content::Seq(vec![
            Content::Seq(vec![
                Content::U32(42),
                Content::Char('a'),
            ]),
            Content::Map(vec![]),
        ]),
    };

    let result = test_content.content.unexpected();
    assert_eq!(result, Unexpected::Seq);
}

