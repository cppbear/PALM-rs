// Answer 0

#[test]
fn test_size_limit_default() {
    let compiler = Compiler::new();
    assert_eq!(compiler.size_limit, 10 * (1 << 20)); // Default size limit
}

#[test]
fn test_size_limit_set_value() {
    let compiler = Compiler::new().size_limit(500);
    assert_eq!(compiler.size_limit, 500); // Check if size limit is set correctly
}

#[test]
fn test_size_limit_zero() {
    let compiler = Compiler::new().size_limit(0);
    assert_eq!(compiler.size_limit, 0); // Check if size limit can be set to 0
}

#[test]
fn test_size_limit_large_value() {
    let compiler = Compiler::new().size_limit(1_000_000);
    assert_eq!(compiler.size_limit, 1_000_000); // Check if a large size limit is accepted
}

