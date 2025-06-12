// Answer 0

#[test]
fn test_unicode_with_yes_true() {
    let mut builder = TranslatorBuilder::new();
    builder.unicode(true);
}

#[test]
fn test_unicode_multiple_calls_yes_true() {
    let mut builder = TranslatorBuilder::new();
    builder.unicode(true).unicode(true);
}

#[test]
fn test_unicode_chained_calls() {
    let mut builder = TranslatorBuilder::new();
    builder.unicode(true).case_insensitive(true).multi_line(true);
}

