// Answer 0

#[test]
fn test_c_byte_min() {
    let mut compiler = Compiler::new();
    compiler.c_byte(0);
}

#[test]
fn test_c_byte_max() {
    let mut compiler = Compiler::new();
    compiler.c_byte(255);
}

#[test]
fn test_c_byte_mid() {
    let mut compiler = Compiler::new();
    compiler.c_byte(128);
}

#[test]
fn test_c_byte_edge() {
    let mut compiler = Compiler::new();
    compiler.c_byte(1);
} 

#[test]
fn test_c_byte_zero() {
    let mut compiler = Compiler::new();
    compiler.c_byte(2);
}

