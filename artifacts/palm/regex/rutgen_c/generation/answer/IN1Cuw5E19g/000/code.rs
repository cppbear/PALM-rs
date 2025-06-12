// Answer 0

#[test]
fn test_size_limit_initialization() {
    let compiler = Compiler::new();
    assert_eq!(compiler.size_limit, 10 * (1 << 20));
}

#[test]
fn test_size_limit_update() {
    let mut compiler = Compiler::new();
    compiler = compiler.size_limit(1024);
    assert_eq!(compiler.size_limit, 1024);
}

#[test]
fn test_size_limit_update_to_zero() {
    let mut compiler = Compiler::new();
    compiler = compiler.size_limit(0);
    assert_eq!(compiler.size_limit, 0);
}

#[test]
fn test_size_limit_update_to_large_value() {
    let mut compiler = Compiler::new();
    compiler = compiler.size_limit(1_000_000);
    assert_eq!(compiler.size_limit, 1_000_000);
}

