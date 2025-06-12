// Answer 0

#[test]
fn test_multi_line_true() {
    let mut builder = TranslatorBuilder::new();
    builder.multi_line(true);
}

#[test]
fn test_multi_line_true_after_allow_invalid_utf8() {
    let mut builder = TranslatorBuilder::new();
    builder.allow_invalid_utf8(true);
    builder.multi_line(true);
}

#[test]
fn test_multi_line_true_after_case_insensitive() {
    let mut builder = TranslatorBuilder::new();
    builder.case_insensitive(true);
    builder.multi_line(true);
}

#[test]
fn test_multi_line_true_after_dot_matches_new_line() {
    let mut builder = TranslatorBuilder::new();
    builder.dot_matches_new_line(true);
    builder.multi_line(true);
}

#[test]
fn test_multi_line_true_after_swap_greed() {
    let mut builder = TranslatorBuilder::new();
    builder.swap_greed(true);
    builder.multi_line(true);
}

#[test]
fn test_multi_line_true_after_unicode() {
    let mut builder = TranslatorBuilder::new();
    builder.unicode(true);
    builder.multi_line(true);
}

