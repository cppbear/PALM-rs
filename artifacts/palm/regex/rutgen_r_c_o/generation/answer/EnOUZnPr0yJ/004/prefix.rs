// Answer 0

#[test]
fn test_c_class_non_empty_ranges() {
    let mut compiler = Compiler::new();
    let ranges = vec![
        hir::ClassUnicodeRange::new(char::from(0), char::from(1)),
        hir::ClassUnicodeRange::new(char::from(2), char::from(3)),
    ];
    let _result = compiler.c_class(&ranges);
}

#[test]
fn test_c_class_multiple_ranges() {
    let mut compiler = Compiler::new();
    let ranges = vec![
        hir::ClassUnicodeRange::new(char::from(0), char::from(2)),
        hir::ClassUnicodeRange::new(char::from(3), char::from(5)),
        hir::ClassUnicodeRange::new(char::from(6), char::from(7)),
    ];
    let _result = compiler.c_class(&ranges);
}

#[test]
fn test_c_class_ranges_with_non_equal_bounds() {
    let mut compiler = Compiler::new();
    let ranges = vec![
        hir::ClassUnicodeRange::new(char::from(0), char::from(3)),
    ];
    let _result = compiler.c_class(&ranges);
}

