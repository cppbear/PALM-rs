// Answer 0

#[test]
fn test_bytes_true() {
    let compiler = Compiler::new().size_limit(5 * (1 << 20));
    let _ = compiler.bytes(true);
}

#[test]
fn test_bytes_false() {
    let compiler = Compiler::new().size_limit(5 * (1 << 20));
    let _ = compiler.bytes(false);
}

#[test]
fn test_bytes_true_with_min_size() {
    let compiler = Compiler::new().size_limit(0);
    let _ = compiler.bytes(true);
}

#[test]
fn test_bytes_false_with_min_size() {
    let compiler = Compiler::new().size_limit(0);
    let _ = compiler.bytes(false);
}

#[test]
fn test_bytes_true_with_max_size() {
    let compiler = Compiler::new().size_limit(10 * (1 << 20));
    let _ = compiler.bytes(true);
}

#[test]
fn test_bytes_false_with_max_size() {
    let compiler = Compiler::new().size_limit(10 * (1 << 20));
    let _ = compiler.bytes(false);
}

#[test]
fn test_bytes_false_with_size_limit_exceed() {
    let compiler = Compiler::new().size_limit(11 * (1 << 20));
    let _ = compiler.bytes(false);
}

