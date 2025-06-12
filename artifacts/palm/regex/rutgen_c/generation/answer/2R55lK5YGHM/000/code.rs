// Answer 0

#[test]
fn test_ignore_whitespace_enable() {
    let mut builder = ParserBuilder::new();
    builder.ignore_whitespace(true);
    assert!(builder.ignore_whitespace);
}

#[test]
fn test_ignore_whitespace_disable() {
    let mut builder = ParserBuilder::new();
    builder.ignore_whitespace(false);
    assert!(!builder.ignore_whitespace);
}

#[test]
fn test_ignore_whitespace_state_persistence() {
    let mut builder = ParserBuilder::new();
    builder.ignore_whitespace(true);
    let builder_ref = builder.ignore_whitespace(false);
    assert!(!builder_ref.ignore_whitespace);
}

