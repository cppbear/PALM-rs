// Answer 0

#[test]
fn test_case_insensitive_false() {
    let mut builder = TranslatorBuilder::new();
    builder.case_insensitive(false);
}

#[test]
fn test_case_insensitive_false_after_initialization() {
    let mut builder = TranslatorBuilder::new().allow_invalid_utf8(true);
    builder.case_insensitive(false);
}

#[test]
fn test_case_insensitive_false_with_multiple_calls() {
    let mut builder = TranslatorBuilder::new();
    builder.case_insensitive(false);
    builder.case_insensitive(false);
}

#[test]
fn test_case_insensitive_false_with_other_flags() {
    let mut builder = TranslatorBuilder::new();
    builder.case_insensitive(false);
    builder.multi_line(true);
    builder.unicode(true);
}

