// Answer 0

#[test]
fn test_c_dotstar_only_utf8() {
    let mut compiler = Compiler::new()
        .size_limit(0)
        .only_utf8(true);
    let _ = compiler.c_dotstar();
}

#[test]
fn test_c_dotstar_large_size_limit() {
    let mut compiler = Compiler::new()
        .size_limit(10485760) // setting size_limit to max
        .only_utf8(true);
    let _ = compiler.c_dotstar();
}

#[test]
fn test_c_dotstar_medium_size_limit() {
    let mut compiler = Compiler::new()
        .size_limit(5242880) // setting size_limit to 5 MB
        .only_utf8(true);
    let _ = compiler.c_dotstar();
}

#[test]
fn test_c_dotstar_minimum_size_limit() {
    let mut compiler = Compiler::new()
        .size_limit(1) // setting size_limit to minimum
        .only_utf8(true);
    let _ = compiler.c_dotstar();
}

