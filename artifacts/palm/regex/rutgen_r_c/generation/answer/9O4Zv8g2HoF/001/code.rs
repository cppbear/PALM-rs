// Answer 0

#[test]
fn test_octal_enable() {
    let mut builder = ParserBuilder::new();
    builder.octal(true);
    assert!(builder.octal);
}

#[test]
fn test_octal_disable() {
    let mut builder = ParserBuilder::new();
    builder.octal(false);
    assert!(!builder.octal);
}

#[test]
fn test_octal_multiple_calls() {
    let mut builder = ParserBuilder::new();
    builder.octal(true);
    assert!(builder.octal);
    builder.octal(false);
    assert!(!builder.octal);
    builder.octal(true);
    assert!(builder.octal);
}

#[test]
fn test_octal_initial_state() {
    let builder = ParserBuilder::new();
    assert!(!builder.octal);
}

