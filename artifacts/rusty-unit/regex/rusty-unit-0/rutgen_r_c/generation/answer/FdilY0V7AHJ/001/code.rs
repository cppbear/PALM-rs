// Answer 0

#[test]
fn test_only_utf8_enabled() {
    let compiler = Compiler::new();
    let result = compiler.only_utf8(true);
    assert_eq!(result.compiled.only_utf8, true);
}

#[test]
fn test_only_utf8_disabled() {
    let compiler = Compiler::new();
    let result = compiler.only_utf8(false);
    assert_eq!(result.compiled.only_utf8, false);
}

#[test]
fn test_only_utf8_chaining() {
    let compiler = Compiler::new();
    let result = compiler.only_utf8(true).only_utf8(false);
    assert_eq!(result.compiled.only_utf8, false);
}

