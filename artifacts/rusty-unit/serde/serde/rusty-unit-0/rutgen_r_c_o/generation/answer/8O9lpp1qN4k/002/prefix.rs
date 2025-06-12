// Answer 0

#[test]
fn test_unexpected_with_seq_content() {
    let content = Content::Seq(vec![
        Content::Bool(true),
        Content::U8(255),
        Content::I32(-1),
        Content::F64(3.14),
        Content::Char('A'),
    ]);
    content.unexpected();
}

#[test]
fn test_unexpected_with_empty_seq_content() {
    let content = Content::Seq(vec![]);
    content.unexpected();
}

#[test]
fn test_unexpected_with_nested_seq_content() {
    let content = Content::Seq(vec![
        Content::Seq(vec![Content::Bool(false)]),
        Content::Seq(vec![Content::I64(42)]),
    ]);
    content.unexpected();
}

