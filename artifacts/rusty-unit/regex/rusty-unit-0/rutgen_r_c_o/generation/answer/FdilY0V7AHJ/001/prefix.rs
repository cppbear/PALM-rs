// Answer 0

#[test]
fn test_only_utf8_false() {
    let compiler = Compiler::new().size_limit(0);
    compiler.only_utf8(false);
}

#[test]
fn test_only_utf8_true() {
    let compiler = Compiler::new().size_limit(10 * (1 << 20));
    compiler.only_utf8(true);
}

#[test]
fn test_only_utf8_with_min_size() {
    let compiler = Compiler::new().size_limit(1);
    compiler.only_utf8(false);
}

#[test]
fn test_only_utf8_with_max_size() {
    let compiler = Compiler::new().size_limit(10 * (1 << 20) - 1);
    compiler.only_utf8(true);
}

