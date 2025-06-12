// Answer 0

#[test]
fn test_case_insensitive_set_false() {
    let mut builder = TranslatorBuilder::new();
    let result = builder.case_insensitive(false);
    assert_eq!(builder.flags.case_insensitive, None);
    assert_eq!(result as *const _, &mut builder as *mut _);
}

#[test]
fn test_case_insensitive_set_true() {
    let mut builder = TranslatorBuilder::new();
    let result = builder.case_insensitive(true);
    assert_eq!(builder.flags.case_insensitive, Some(true));
    assert_eq!(result as *const _, &mut builder as *mut _);
}

