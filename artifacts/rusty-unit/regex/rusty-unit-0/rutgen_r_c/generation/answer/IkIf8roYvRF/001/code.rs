// Answer 0

#[test]
fn test_case_insensitive_true() {
    let mut builder = TranslatorBuilder::new();
    let result = builder.case_insensitive(true);
    assert_eq!(result.flags.case_insensitive, Some(true));
}

#[test]
fn test_case_insensitive_false() {
    let mut builder = TranslatorBuilder::new();
    let result = builder.case_insensitive(false);
    assert_eq!(result.flags.case_insensitive, None);
}

