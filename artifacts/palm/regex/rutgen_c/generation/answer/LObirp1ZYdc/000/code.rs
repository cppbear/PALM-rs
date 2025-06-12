// Answer 0

#[test]
fn test_multi_line_enable() {
    let mut builder = TranslatorBuilder::new();
    builder.multi_line(true);
    assert_eq!(builder.flags.multi_line, Some(true));
}

#[test]
fn test_multi_line_disable() {
    let mut builder = TranslatorBuilder::new();
    builder.multi_line(false);
    assert_eq!(builder.flags.multi_line, None);
}

#[test]
fn test_multi_line_no_change() {
    let mut builder = TranslatorBuilder::new();
    builder.multi_line(true);
    builder.multi_line(false);
    assert_eq!(builder.flags.multi_line, None);
}

