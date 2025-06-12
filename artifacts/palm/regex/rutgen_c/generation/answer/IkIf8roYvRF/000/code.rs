// Answer 0

#[test]
fn test_case_insensitive_enable() {
    let mut builder = TranslatorBuilder::new();
    builder.case_insensitive(true);
    assert_eq!(builder.flags.case_insensitive, Some(true));
}

#[test]
fn test_case_insensitive_disable() {
    let mut builder = TranslatorBuilder::new();
    builder.case_insensitive(false);
    assert_eq!(builder.flags.case_insensitive, None);
} 

#[test]
fn test_case_insensitive_multiple_calls() {
    let mut builder = TranslatorBuilder::new();
    builder.case_insensitive(true);
    builder.case_insensitive(false);
    assert_eq!(builder.flags.case_insensitive, None);
}

