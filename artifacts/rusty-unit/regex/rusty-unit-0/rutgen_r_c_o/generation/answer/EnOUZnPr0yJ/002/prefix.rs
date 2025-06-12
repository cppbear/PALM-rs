// Answer 0

#[test]
fn test_c_class_with_single_range() {
    let mut compiler = Compiler::new().bytes(true);
    let ranges = vec![hir::ClassUnicodeRange::new('a', 'a')];
    compiler.c_class(&ranges).unwrap();
}

#[test]
fn test_c_class_with_multiple_ranges() {
    let mut compiler = Compiler::new().bytes(true);
    let ranges = vec![
        hir::ClassUnicodeRange::new('a', 'b'),
        hir::ClassUnicodeRange::new('d', 'e'),
        hir::ClassUnicodeRange::new('g', 'z'),
    ];
    compiler.c_class(&ranges).unwrap();
}

#[test]
fn test_c_class_with_edge_range() {
    let mut compiler = Compiler::new().bytes(true);
    let ranges = vec![hir::ClassUnicodeRange::new('\u{10FFFF}', '\u{10FFFF}')];
    compiler.c_class(&ranges).unwrap();
}

#[test]
fn test_c_class_with_full_unicode_range() {
    let mut compiler = Compiler::new().bytes(true);
    let ranges = vec![
        hir::ClassUnicodeRange::new('\u{0000}', '\u{10FFFF}')
    ];
    compiler.c_class(&ranges).unwrap();
}

#[test]
fn test_c_class_with_multiple_single_char_ranges() {
    let mut compiler = Compiler::new().bytes(true);
    let ranges = (0..100).map(|c| hir::ClassUnicodeRange::new(char::from(c as u32), char::from(c as u32))).collect::<Vec<_>>();
    compiler.c_class(&ranges).unwrap();
}

