// Answer 0

#[test]
fn test_dot_matches_new_line_enabled() {
    let mut builder = TranslatorBuilder::new();
    builder.dot_matches_new_line(true);
    assert_eq!(builder.flags.dot_matches_new_line, Some(true));
}

#[test]
fn test_dot_matches_new_line_disabled() {
    let mut builder = TranslatorBuilder::new();
    builder.dot_matches_new_line(false);
    assert_eq!(builder.flags.dot_matches_new_line, None);
}

