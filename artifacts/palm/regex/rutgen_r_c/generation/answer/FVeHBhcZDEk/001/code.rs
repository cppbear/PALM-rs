// Answer 0

#[test]
fn test_c_byte_valid() {
    let mut compiler = Compiler::new();
    let result = compiler.c_byte(0xFF);
    assert!(result.is_ok());
}

#[test]
fn test_c_byte_zero() {
    let mut compiler = Compiler::new();
    let result = compiler.c_byte(0x00);
    assert!(result.is_ok());
}

#[test]
fn test_c_byte_mid_range() {
    let mut compiler = Compiler::new();
    let result = compiler.c_byte(0x80);
    assert!(result.is_ok());
}

#[test]
fn test_c_byte_boundary() {
    let mut compiler = Compiler::new();
    let result = compiler.c_byte(0x01);
    assert!(result.is_ok());
}

