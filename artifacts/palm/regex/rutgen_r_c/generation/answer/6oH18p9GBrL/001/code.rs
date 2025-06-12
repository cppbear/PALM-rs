// Answer 0

#[test]
fn test_only_utf8_true() {
    let mut builder = ExecBuilder::new("test");
    builder = builder.only_utf8(true);
    assert!(builder.only_utf8); // Verify that the option was set correctly
}

#[test]
fn test_only_utf8_false() {
    let mut builder = ExecBuilder::new("test");
    builder = builder.only_utf8(false);
    assert!(!builder.only_utf8); // Verify that the option was set correctly
}

#[test]
fn test_only_utf8_multiple() {
    let mut builder = ExecBuilder::new_many(vec!["a", "b", "c"]);
    builder = builder.only_utf8(true);
    assert!(builder.only_utf8); // Confirm it works with multiple patterns
}

#[test]
fn test_only_utf8_no_change() {
    let mut builder = ExecBuilder::new("test");
    let original_builder = builder.clone();
    builder = builder.only_utf8(original_builder.only_utf8); 
    assert_eq!(builder, original_builder); // Confirm it retains the original state when setting the same value
}

