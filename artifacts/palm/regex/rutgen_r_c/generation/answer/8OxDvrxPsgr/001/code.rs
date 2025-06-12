// Answer 0

#[test]
fn test_unicode_flag_enabled() {
    let mut builder = TranslatorBuilder::new();
    
    // Test enabling Unicode flag (yes = true)
    let result = builder.unicode(true);
    assert_eq!(result, &mut builder);
    assert_eq!(builder.flags.unicode, None);
}

#[test]
fn test_unicode_flag_disabled() {
    let mut builder = TranslatorBuilder::new();
    
    // Test disabling Unicode flag (yes = false)
    let result = builder.unicode(false);
    assert_eq!(result, &mut builder);
    assert_eq!(builder.flags.unicode, Some(false));
}

