// Answer 0

#[test]
fn test_bytes_function_when_yes_is_true() {
    let mut builder = ExecBuilder::new("test");
    builder = builder.bytes(true);
    assert!(builder.bytes);
}

#[test]
fn test_bytes_function_when_yes_is_false() {
    let mut builder = ExecBuilder::new("test");
    builder = builder.bytes(false);
    assert!(!builder.bytes);
}

#[test]
fn test_multiple_calls_to_bytes_function() {
    let mut builder = ExecBuilder::new("test");
    builder = builder.bytes(false);
    assert!(!builder.bytes);
    
    builder = builder.bytes(true);
    assert!(builder.bytes);
    
    builder = builder.bytes(false);
    assert!(!builder.bytes);
}

