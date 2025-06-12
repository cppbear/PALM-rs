// Answer 0

#[test]
fn test_c_class_with_no_ranges() {
    let mut compiler = Compiler::new();
    let result = compiler.c_class(&[]);
    assert!(result.is_err());
}

#[test]
fn test_c_class_with_uses_bytes() {
    let mut compiler = Compiler::new().bytes(true);
    let ranges = vec![hir::ClassUnicodeRange::new('a', 'z')];
    let result = compiler.c_class(&ranges);
    assert!(result.is_ok());
}

#[test]
fn test_c_class_with_one_range_equal() {
    let mut compiler = Compiler::new();
    let ranges = vec![hir::ClassUnicodeRange::new('a', 'a')];
    let result = compiler.c_class(&ranges);
    assert!(result.is_ok());
}

#[test]
fn test_c_class_with_multiple_ranges_equal() {
    let mut compiler = Compiler::new();
    let ranges = vec![
        hir::ClassUnicodeRange::new('a', 'c'),
        hir::ClassUnicodeRange::new('e', 'g'),
    ];
    let result = compiler.c_class(&ranges);
    assert!(result.is_ok());
}

#[test]
fn test_c_class_with_two_ranges() {
    let mut compiler = Compiler::new();
    let ranges = vec![
        hir::ClassUnicodeRange::new('a', 'c'),
        hir::ClassUnicodeRange::new('d', 'f'),
    ];
    let result = compiler.c_class(&ranges);
    assert!(result.is_ok());

    if let Ok(patch) = result {
        assert!(patch.hole != Hole::None);
        assert_eq!(patch.entry, compiler.insts.len() - 1);
    }
}

